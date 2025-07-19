# arroyo-soc-udf: Security Operations Center User Defined Functions

This project demonstrates how to create a simple User Defined Function (UDF) for Arroyo 0.14+, using the new C‑ABI-based plugin system. The plan includes a full integration test that starts an Arroyo instance in-process, dynamically loads the UDF, and validates its behavior. The integration test is designed to run in GitHub Actions CI.

---

For all unit and integration tests write the test first, pausing for me to review the test before you write the implementation. Name the tests in a way that describes the behavior of the code. Code as if you are a Behavior Driven Development practitioner.

Most UDF tests should be unit tests, located near the code that they test.

## Implementation Steps

[ ] Step 1: Initialize a new Rust crate for the UDF (`arroyo-soc-udf`)
- cargo init for an Arroyo UDF plugin dynamic library
- Same license file as Arroyo is present
- Create `.github/workflows/ci.yml` to build the dynamic library
- CI should be required for all GitHub PRs

[ ] Step 2: CI build runs an integration test that loads the Arroyo UDF plugin dynamic library

[ ] Step 3: Add a "split" function exported by the Arroyo UDF plugin dynamic library
- The function should accept two strings, it should have the same semantics as the "split" fucntion in JavaScript
- The function should return a vector of strings

[ ] Step 4: Modify the existing integration test to run the Arroyo engine in process, loading the UDF plugin dynamically in Arroyo instead

[ ] Step 5: Modify the existing integration test to run a streaming query
- the integration test should write strings to the input source and expect to read an array of one string the output sink
- write a SQL stream query that makes the test pass, i.e. just puts the input string into a vector and returns it

[ ] Step 6: Modify the existing integration test to run a streaming query that uses the split UDF
- the integration test should write strings to the input source and expect to read an array of one string the output sink
-  the SQL stream query should use the split UDF with a '.' character as the delimiter, so set up the test to include combinations of '.' and non '.' characters