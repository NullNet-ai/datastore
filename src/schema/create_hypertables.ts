import * as fs from 'fs';
import * as child_process from 'child_process';
var path = require('path');

/**
 * Static file path for the `index.ts`.
 */
const INDEX_FILE_PATH = path.join(
  __dirname,
  '../../src/schema/application/index.ts',
);

const DRIZZLE_DIR_PATH = path.join(process.cwd(), 'drizzle');

/**
 * Parses the static `index.ts` file to extract uncommented table exports.
 * @returns List of table names that are uncommented.
 */
function getUncommentedTables() {
  try {
    // Check if the file exists
    if (!fs.existsSync(INDEX_FILE_PATH)) {
      console.error(`File not found: ${INDEX_FILE_PATH}`);
      return [];
    }

    // Read the contents of the file
    const fileContents = fs.readFileSync(INDEX_FILE_PATH, 'utf-8');

    // Regex to match uncommented table exports
    const tableRegex =
      /^\s*export\s+\{[^}]*\s+as\s+(\w+)\s*\}\s+from\s+['"].*['"]\s*;$/gm;
    const matches = Array.from(fileContents.matchAll(tableRegex));

    // Extract and return table names from matches
    return matches.map((match) => match[1]);
  } catch (error) {
    console.error(
      `Error reading or parsing file at ${INDEX_FILE_PATH}:`,
      error,
    );
    return [];
  }
}
/**
 * Deletes all files with "hypertables" in their name except the latest one.
 */
// @ts-ignore

function cleanHypertableFiles() {
  try {
    const files = fs
      .readdirSync(DRIZZLE_DIR_PATH)
      .filter((file) => file.includes('hypertables') && file.endsWith('.sql'));

    if (files.length < 1) return;

    // Sort files numerically based on their prefix (e.g., 0001, 0002, etc.)
    const sortedFiles = files.sort((a, b) => {
      const numA = parseInt(a.split('_')[0] as string, 10);
      const numB = parseInt(b.split('_')[0] as string, 10);
      return numA - numB;
    });

    // Keep the latest file and delete the rest
    for (let i = 0; i <= sortedFiles.length - 1; i++) {
      const filePath = path.join(DRIZZLE_DIR_PATH, sortedFiles[i]);
      fs.unlinkSync(filePath);
      console.log(`Deleted old hypertables file: ${filePath}`);
    }
  } catch (error) {
    console.error('Error cleaning hypertable files:', error);
  }
}

/**
 * Write CREATE_HYPERTABLE queries to the latest SQL file.
 * @param tableNames List of table names
 */
function writeHypertableQueriesToDrizzleFile(tableNames: string[]) {
  try {
    // Ensure the drizzle directory exists
    if (!fs.existsSync(DRIZZLE_DIR_PATH)) {
      console.error(`Drizzle directory not found at ${DRIZZLE_DIR_PATH}`);
      return;
    }

    // Find the latest SQL file with "hypertables" in its name
    const sqlFiles = fs
      .readdirSync(DRIZZLE_DIR_PATH)
      .filter((file) => file.includes('hypertables') && file.endsWith('.sql'));

    if (sqlFiles.length === 0) {
      console.error(`No SQL files found in ${DRIZZLE_DIR_PATH}`);
      return;
    }

    // Sort files numerically based on their prefix
    const sortedFiles = sqlFiles.sort((a: string, b: string) => {
      const numA = parseInt(a.split('_')[0] as any, 10);
      const numB = parseInt(b.split('_')[0] as any, 10);
      return numA - numB;
    });

    const latestFilePath = path.join(
      DRIZZLE_DIR_PATH,
      sortedFiles[sortedFiles.length - 1],
    );

    // Prepare SQL queries for each table
    const queries = tableNames
      .map(
        (tableName) =>
          `SELECT create_hypertable('${tableName}', 'time', chunk_time_interval => INTERVAL '7 days', if_not_exists => TRUE);`,
      )
      .join('\n');

    // Append the queries to the latest SQL file
    fs.appendFileSync(
      latestFilePath,
      `\n-- Hypertable Creation Queries\n${queries}\n`,
    );
    console.log(`Hypertable queries written to ${latestFilePath}`);
  } catch (error) {
    console.error('Error writing to the SQL file:', error);
  }
}

/**
 * Main function to process the tables and write queries.
 */
function processHypertableQueries() {
  const tableNames: any = getUncommentedTables();

  if (tableNames.length === 0) {
    console.warn('No uncommented tables found.');
    return;
  }

  try {
    // const hypertableFiles = fs
    //   .readdirSync(DRIZZLE_DIR_PATH)
    //   .filter((file) => file.includes('hypertables') && file.endsWith('.sql'));
    //
    // // Run the drizzle:generate-hypertables command
    // // If files exist, clean them
    // if (hypertableFiles.length > 0) {
    //   console.log('Cleaning old hypertable files...');
    //   cleanHypertableFiles();
    // }
    console.log('Running command: npm run drizzle:generate-hypertables');
    child_process.execSync('npm run drizzle:generate-hypertables', {
      stdio: 'inherit',
    });
    // Write queries to the latest SQL file
    writeHypertableQueriesToDrizzleFile(tableNames);
  } catch (error) {
    console.error(
      'Error running drizzle:generate-hypertables or processing files:',
      error,
    );
  }
}

export { processHypertableQueries };
