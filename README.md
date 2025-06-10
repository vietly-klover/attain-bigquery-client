Check https://bigquery.googleapis.com/$discovery/rest?version=v2 for documentation

## Running Integration Tests

Some integration tests are ignored by default (e.g., tests that require a GCP environment). To run all tests, including ignored ones, use:

```
cargo test -- --ignored
```

To run only the ignored tests:

```
cargo test -- --ignored --include-ignored
```

### Running a Specific Ignored Integration Test

For example, to run the ignored test in `imds_config_provider_integration.rs`:

```
cargo test test_grab_default_project_id -- --ignored
```

Note: Some tests may require specific environment setup (e.g., GCP metadata server access). See comments in the test files for details.