# Solana TX Data Extract, Storage, API Code Challenge

## Objective

Build a simple Rust application that interacts with the Solana blockchain to process and store transaction data and provide a queryable API endpoint.

## Challenge Details

### 1. **Fetch and Process Blockchain Data**
Create an application in Rust that:
- **Interacts with the Solana blockchain** to fetch transaction data from the [Phoenix on-chain orderbook DEX](https://www.phoenix.trade/) program.
- **Decodes transactions** and extracts trade fill events.
- **Stores the processed data** in a database of your choice (Postgres or ClickHouse preferred).
- Runs **continuously** to fetch new transactions, parse them, and insert the extracted trade fill events into the database in real-time or near real-time.

### 2. **Build an API Endpoint**
Develop a RESTful API in Rust to query the processed data:
- The API should return data in [OHLC format (Open, High, Low, Close)](https://www.investopedia.com/terms/o/ohlcchart.asp).
- **Endpoint Requirements:**
  - Accept the following request parameters:
    - `baseTokenMint`: The mint address of the base token.
    - `quoteTokenMint`: The mint address of the quote token.
    - `startTime` and `endTime`: Specify the time range for the query (e.g., in UNIX timestamp format).
    - `interval/resolution`: Specify the interval for aggregation (e.g., 1m, 1h, 1d, etc.).
  - Validate input parameters and return meaningful error messages for invalid requests.
  - Support efficient queries from the database to return the requested OHLC data.

### 3. **Database Considerations**
- Design a database schema that supports:
  - Storage of trade fill events.
  - Efficient querying for OHLC data based on the parameters.
- Populate the database with **dummy data** for testing if necessary. Ensure the database schema includes an **additional column** to differentiate between real and dummy data.

## Bonus Tasks (not required)
- **Rate Limiting:** Implement rate limiting on the API (e.g., limit requests per minute per client IP).
- **Credit Management:** Add a basic credit management system where:
  - Each API request deducts 1 credit in real time.
  - The system returns an appropriate error if the client has insufficient credits.

## RPC API & Resources

You can use any RPC provider of your choice, but we recommend the following approach for convenience:

1. Visit the [Helius Dev Portal](https://dev.helius.xyz/dashboard/app), log in with your preferred method, and retrieve your free API key.
2. Set this API key as an environment variable in your application.
3. Use the endpoint `https://rpc.helius.xyz/?api-key=${YOUR_API_KEY}` to fetch data.  
   - **Note**: This endpoint has a rate limit of **25 RPC requests per second**.

To fetch transactions for the Phoenix DEX Program:
- Use the [getSignaturesForAddress](https://docs.solana.com/api/http#getsignaturesforaddress) method to get a list of transaction signatures.
- Decode these transactions to extract the relevant trade fill events.

For decoding and understanding the Phoenix DEX data:
- The [Phoenix SDK repository](https://github.com/Ellipsis-Labs/phoenix-sdk/tree/master) provides helpful insights into the data structures and logic for identifying and extracting trade fill events.

These are suggested guidelines to help you get started—you are welcome to use alternative approaches if preferred.


## Testing (Optional but Recommended)

While not mandatory, it is encouraged to include a basic suite of tests that cover the core functionality of your application.

- **Focus Areas**: Test critical paths such as blockchain interaction, data decoding, and the OHLC endpoint logic.
- **Test Coverage**: 100% code coverage is not required but demonstrating thoughtful testing is a plus.

## Development and Build Environment

Your submission should include a fully working development environment. Ensure you provide:
- Clear instructions for setting up the application.
- Steps for installing dependencies and running the application in a development environment.
- **Development Tools**: You are welcome to use tools like Docker to simplify setup and manage dependencies if it makes the process easier.

## Submission Guidelines

Please submit your work via a GitHub repository. Your submission should include:

1. **Source Code**: The complete application codebase.
2. **Test Files**: Include any test files and instructions to run the tests, if applicable.
3. **README File**: A README containing:
   - Detailed setup instructions, including how to:
     - Install dependencies.
     - Run the application.
     - Execute tests (if applicable).
   - Instructions for using the application.
4. **Database with Trade Fills Data**: Include a database containing the processed trade fills data.
5. **Additional Documentation**:
   - Explain your assumptions.
   - Describe your architecture decisions.
   - Highlight any notable implementation details.


## Expectations and Time Guidelines

- **Estimated Time**: This challenge is estimated to take approximately **4-6 hours** to complete the core requirements. This is not a hard limit but rather a guideline to help you plan your time. If you find yourself significantly exceeding this estimate, feel free to let us know.
- **Evaluation Criteria**:
  - Your approach to solving the problem.
  - Code quality and maintainability.
  - Choice of libraries/tools.
  - Adherence to best practices.

Good luck, and we look forward to your submission!
