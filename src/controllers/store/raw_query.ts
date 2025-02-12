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

          // PostgreSQL null representation
          if (value === null) return 'null';

          // Undefined values are treated as empty fields
          if (value === undefined) return null;

          // Convert timestamps to ISO strings if the key contains "timestamp"
          if (
            key.toLowerCase().includes('timestamp') &&
            value instanceof Date
          ) {
            return `"${value.toISOString()}"`;
          }
          if (
            key.toLowerCase().includes('timestamp') &&
            typeof value === 'string'
          ) {
            try {
              return `"${new Date(value).toISOString()}"`;
            } catch {
              return value; // Keep as-is if it's not a valid date
            }
          }

          // Handle strings with escaping for CSV format
          if (typeof value === 'string') {
            return `"${value.replace(/"/g, '""')}"`;
          }

          // For all other types (number, boolean, etc.), keep as-is
          return value;
        })
        .join(',');
      this.push(csvRow + '\n');
      callback();
    },
  });
}

// Function to copy data from a JSON array to a PostgreSQL table
export async function copyData(
  tableName: string,
  jsonData: any[],
  table_columns: string[],
) {
  if (!jsonData || jsonData.length === 0) {
    throw new Error('No data to copy.');
  }

  const client = await pool.connect();
  try {
    await client.query('BEGIN');

    // Wrap the copyData calls in Promises
    const copyMainPromise = copyDataToTable(
      client,
      tableName,
      jsonData,
      table_columns,
    );
    const copyTempPromise = copyDataToTable(
      client,
      `temp_${tableName}`,
      jsonData,
      table_columns,
    );

    // Execute both COPY operations concurrently
    await Promise.all([copyMainPromise, copyTempPromise]);

    await client.query('COMMIT');
    console.log('Data successfully copied to both tables!');
  } catch (error) {
    console.error('Error during COPY operation:', error);
    await client.query('ROLLBACK');
  } finally {
    client.release();
  }
}

async function copyDataToTable(
  client: any,
  tableName: string,
  jsonData: any[],
  table_columns: string[],
) {
  if (!jsonData || jsonData.length === 0) {
    throw new Error('No data to copy.');
  }

  try {
    // Dynamically determine column order from the first record

    // Create a COPY query with explicit column names
    const copyQuery = `COPY ${tableName} (${table_columns.join(
      ',',
    )}) FROM STDIN CSV HEADER`;

    const stream = client.query(copyFrom(copyQuery));
    const transformer = jsonToCsvTransform(table_columns);

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

    console.log(`Data successfully copied to table ${tableName}!`);
  } catch (error) {
    console.error(`Error during COPY operation to table ${tableName}:`, error);
    throw error; // Re-throw the error to be caught by the caller
  }
}
