const { Pool } = require('pg');
const { Transform } = require('stream');
const copyFrom = require('pg-copy-streams').from;

const pool = new Pool({
  user: 'admin',
  host: 'localhost',
  database: 'nullnet',
  password: 'admin',
  port: 5432,
});

// Helper function to create a Transform stream for converting JSON to CSV
function jsonToCsvTransform(columnOrder) {
  let isFirstChunk = true;

  return new Transform({
    writableObjectMode: true,
    readableObjectMode: false,
    transform(record, _encoding, callback) {
      if (isFirstChunk) {
        // Push the header row (columns) based on the specified column order
        this.push(columnOrder.join(',') + '\n');
        isFirstChunk = false;
      }

      // Map the values in the record to the column order
      const csvRow = columnOrder
        .map((key) => {
          const value = record[key];
          if (value === null) return 'null'; // PostgreSQL null representation
          if (value === undefined) return ''; // Handle undefined as an empty field
          if (key === 'timestamp') return new Date(value).toISOString();
          if (typeof value === 'string') {
            return `"${value.replace(/"/g, '""')}"`; // Escape double quotes in strings
          }
          return value.toString(); // Convert other types to strings
        })
        .join(',');
      this.push(csvRow + '\n');
      callback();
    },
  });
}

// Function to copy data from a JSON array to a PostgreSQL table
export async function copyData(tableName, jsonData) {
  if (!jsonData || jsonData.length === 0) {
    throw new Error('No data to copy.');
  }

  const client = await pool.connect();
  try {
    await client.query('BEGIN');

    // Dynamically determine column order from the first record
    const columnOrder = Object.keys(jsonData[0]);

    // Create a COPY query with explicit column names
    const copyQuery = `COPY ${tableName} (${columnOrder.join(
      ',',
    )}) FROM STDIN CSV HEADER`;

    const stream = client.query(copyFrom(copyQuery));
    const transformer = jsonToCsvTransform(columnOrder);

    // Simulate a JSON stream
    const jsonStream = new Transform({
      objectMode: true,
      transform(chunk, _encoding, done) {
        this.push(chunk);
        done();
      },
    });

    jsonData.forEach((obj) => jsonStream.push(obj));
    jsonStream.end();

    // Pipe the JSON data through the transformer to the PostgreSQL COPY stream
    jsonStream.pipe(transformer).pipe(stream);

    await new Promise((resolve, reject) => {
      stream.on('finish', resolve);
      stream.on('error', reject);
    });

    await client.query('COMMIT');
    console.log('Data successfully copied to PostgreSQL!');
  } catch (error) {
    console.error('Error during COPY operation:', error);
    await client.query('ROLLBACK');
  } finally {
    client.release();
  }
}
