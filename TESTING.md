# Testing Documentation

This document provides detailed information about the test suite for the Cyberlight project.

## Test Organization

Tests are organized in the `src/tests/` directory with the following structure:

```
src/tests/
├── test_constants/          # Tests for constants and enums
│   └── test_enums.rs
├── test_error_handlers/     # Tests for error handling
│   ├── test_error_handlers.rs
│   └── test_error_implementations.rs
├── test_implementations/    # Tests for implementations (e.g., auth)
│   └── test_access_token.rs
├── test_routes/            # Tests for HTTP routes
│   ├── test_all_devices_routes.rs
│   ├── test_healthcheck_routes.rs
│   └── test_home_routes.rs
├── test_services/          # Tests for service layer
│   └── test_light_setup_service.rs
└── test_wrappers/          # Tests for wrapper functions
    └── test_all_devices_wrapper.rs
```

## Test Categories

### Unit Tests (No External Dependencies)

These tests run without requiring environment variables or external API access:

1. **Wrapper Tests** (`test_wrappers/`)
   - `test_wrap_devices`: Tests device data transformation
   - `test_wrap_model_and_devices`: Tests model and device mapping
   - `test_wrap_device_status`: Tests device status filtering

2. **Error Implementation Tests** (`test_error_handlers/test_error_implementations.rs`)
   - Serialization/deserialization of error types
   - AuthError, NotFoundError, ServerError handling

3. **Service Tests** (`test_services/`)
   - `test_office_setup_creates_payload_with_on_command`: Tests on command payload creation
   - `test_office_setup_creates_payload_with_off_command`: Tests off command payload creation
   - `test_office_setup_handles_all_device_types`: Tests all device type variants

4. **Constants Tests** (`test_constants/`)
   - Device struct creation and field access
   - OfficeDevices enum variants
   - All device types can be instantiated

5. **Route Tests** (`test_routes/`)
   - `test_healthcheck_handler`: Tests healthcheck endpoint response
   - `test_home_redirects_to_status`: Tests home route redirect behavior
   - Error handler tests (404, 401, 500)

### Integration Tests (Require Environment Setup)

These tests are marked with `#[ignore]` and require environment variables:

1. **API Integration Tests** (`test_routes/test_all_devices_routes.rs`)
   - `test_get_all_devices_handler`: Requires `GOVEE_API_KEY`
   - `test_get_status_for_all_devices`: Requires `GOVEE_API_KEY`
   - `test_get_status_for_device`: Requires `GOVEE_API_KEY`

2. **Authorization Tests** (`test_implementations/test_access_token.rs`)
   - `test_valid_authorization_token`: Requires `ACCESS_TOKEN`, `GOVEE_API_KEY`
   - `test_missing_authorization_header`: Requires `ACCESS_TOKEN`
   - `test_invalid_authorization_token`: Requires `ACCESS_TOKEN`

## Running Tests

### Run All Unit Tests
```bash
cargo test
```
Expected output: 20+ tests passing, 6 tests ignored

### Run Specific Test Module
```bash
# Run wrapper tests only
cargo test test_wrap

# Run error handler tests only
cargo test test_error

# Run service tests only
cargo test test_light_setup
```

### Run Integration Tests

First, set up environment variables (create a `.env` file or export them):

```bash
# Required for API integration tests
export GOVEE_API_KEY="your_govee_api_key"
export GOVEE_ROOT_URL="https://developer-api.govee.com"
export ACCESS_TOKEN="your_access_token"

# Required for device control tests
export OFFICE_BOARD_LED_ID="your_board_led_id"
export OFFICE_BOARD_LED_MODEL="your_board_led_model"
export OFFICE_TABLE_LED_ID="your_table_led_id"
export OFFICE_TABLE_LED_MODEL="your_table_led_model"
export OFFICE_WINDOW_LED_ID="your_window_led_id"
export OFFICE_WINDOW_LED_MODEL="your_window_led_model"
export OFFICE_STANDING_LEFT_LED_ID="your_standing_left_led_id"
export OFFICE_STANDING_LEFT_LED_MODEL="your_standing_left_led_model"
export OFFICE_STANDING_RIGHT_LED_ID="your_standing_right_led_id"
export OFFICE_STANDING_RIGHT_LED_MODEL="your_standing_right_led_model"
export OFFICE_HUMIDIFIER_ID="your_humidifier_id"
export OFFICE_HUMIDIFIER_MODEL="your_humidifier_model"
```

Then run the ignored tests:

```bash
# Run only ignored tests
cargo test -- --ignored

# Run all tests (including ignored ones)
cargo test -- --include-ignored
```

## Test Maintenance

When adding new features, please ensure:

1. **Unit tests** are added for pure functions and logic that doesn't require external dependencies
2. **Integration tests** are added for API endpoints and routes
3. Tests requiring environment variables are marked with `#[ignore = "reason"]`
4. Test names clearly describe what is being tested
5. Tests are organized in the appropriate test module

## Coverage Improvements Made

The recent test suite enhancements include:

- ✅ Uncommented and fixed wrapper tests (3 tests)
- ✅ Added home route redirect test (1 test)
- ✅ Added service layer tests (3 tests)
- ✅ Added error implementation tests (6 tests)
- ✅ Added constants/enums tests (3 tests)
- ✅ Marked integration tests as ignored when environment variables are not set
- ✅ Organized tests into logical modules

Total: 20+ passing unit tests, 6 integration tests (ignored by default)

## Continuous Integration

In CI/CD pipelines, you can:

1. Run unit tests without environment setup: `cargo test`
2. Set up environment secrets and run all tests: `cargo test -- --include-ignored`

## Future Test Improvements

Potential areas for additional test coverage:

- Mock external API calls for integration tests
- Add performance/benchmark tests for critical paths
- Add property-based tests for data transformations
- Add tests for edge cases and error conditions in routes
- Add tests for concurrent request handling
