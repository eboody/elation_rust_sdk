# [[📌 Project Todos]]:

- [ ] Implemented the Client Module (libs/client/):

Created the Client struct to handle HTTP requests and authentication.
Implemented HTTP methods (get, post, put, delete) with proper error handling.
Added support for query parameters by introducing the Params trait.
Defined custom error handling in client::error.rs with an Error enum covering various HTTP and parsing errors.
Developed Data Models (libs/models/):

- [ ] Created data models for patients in patient_profile.rs.
Defined structs like Patient, PatientForCreate, PatientQueryParams, etc.
Implemented enums for categorical data (e.g., Sex, GenderIdentity, Race, Ethnicity).
Used serde for serialization and deserialization, with custom date formats using the time crate.
Ensured all models implement necessary traits like Serialize, Deserialize, and Default.
Built the Services Module (libs/services/):

- [ ] Implemented PatientService with methods to interact with the patient API endpoints:
get_patient
create_patient
update_patient
delete_patient
find_patients
Handled errors appropriately by wrapping client::Error in services::Error and adding service-specific error variants.
Used the Params trait to handle query parameters in service methods.
Wrote Unit Tests for PatientService:

- [ ] Used httpmock to mock HTTP responses and simulate API interactions in tests.
Covered success scenarios for all service methods.
Tested error handling by simulating API errors like 404 Not Found.
Ensured all tests pass successfully, verifying the correctness of service methods.
Managed Error Handling Across Modules:

- [ ] Avoided duplication of error definitions by reusing client::Error in the services crate.
Wrapped client::Error in services::Error to propagate client errors while allowing for service-specific errors.
Implemented error conversion and display logic to provide meaningful error messages.
Enhanced the Client for Query Parameters:

- [ ] Modified the Client struct to handle query parameters effectively.
Added a generic get method that accepts parameters implementing the Params trait.
Defined the Params trait requiring Serialize and Default implementations.
Applied the Params trait to parameter structs like PatientQueryParams.
Documented Code and Modules:

- [ ] Added documentation comments (///) to all public structs, enums, methods, and functions.
Provided examples where helpful to illustrate usage.
Ensured code is well-documented for better maintainability and user understanding.
Outlined the Project Structure:

- [ ] Created a detailed markdown outline of the entire project.
Included descriptions of each module and their key components.
Provided a file and module hierarchy for visualization.
Prepared the outline for use in note-taking apps like Obsidian.
# Outline
## Overview

This project is a Rust SDK for interacting with a medical API. It includes modules for handling clients, services, models, errors, and tests. The SDK is designed to be robust, type-safe, and easy to use.

---

## Project Structure

- `Cargo.toml` - Project manifest file, specifying dependencies and project metadata.
- `libs/` - Directory containing library crates for the SDK.
  - `client/` - Client library for making HTTP requests.
  - `services/` - Service library containing business logic and API interactions.
  - `models/` - Data models representing API resources.
  - `utils/` - Utility functions and helpers.

---

## Detailed Structure

### 1. `libs/`

#### 1.1. `client/`

- **Description**: The `client` crate handles HTTP requests, authentication, and error handling.

- **Files**:
  - `src/lib.rs` - Main library file exporting the `Client` struct and its methods.
  - `src/error.rs` - Defines the `Error` enum for client-specific errors.
  - `src/params.rs` - Defines the `Params` trait and related functionality.
  - `src/response.rs` - Handles HTTP responses and error mapping.

- **Key Components**:
  - **Client Struct**:
    - Handles HTTP methods (`get`, `post`, `put`, `delete`).
    - Manages the base URL and authentication tokens.
    - Implements methods for making requests with or without parameters.
  - **Error Handling**:
    - Custom `Error` enum representing possible client errors.
    - Implements `From` traits for error conversions.
  - **Params Trait**:
    - Used for serializing query parameters in requests.
    - Requires `Serialize` and `Default`.

#### 1.2. `services/`

- **Description**: The `services` crate contains services for interacting with API resources, encapsulating business logic.

- **Files**:
  - `src/lib.rs` - Main library file exporting service structs.
  - `src/error.rs` - Defines the `Error` enum for service-specific errors, wrapping `client::Error`.
  - `src/patient_service.rs` - Service for patient-related API interactions.
  - `src/problem_service.rs` - (Planned) Service for problem-related API interactions.
  - `tests/` - Directory containing tests for services.

- **Key Components**:
  - **PatientService**:
    - Methods:
      - `new` - Creates a new instance of the service.
      - `get_patient` - Retrieves a patient by ID.
      - `create_patient` - Creates a new patient.
      - `update_patient` - Updates an existing patient.
      - `delete_patient` - Deletes a patient by ID.
      - `find_patients` - Finds patients based on query parameters.
    - Error Handling:
      - Uses `Error` enum to represent possible errors.
      - Wraps `client::Error` and adds service-specific errors.
    - Testing:
      - Comprehensive tests using `httpmock` to simulate API responses.

#### 1.3. `models/`

- **Description**: The `models` crate contains data models representing API resources, including serialization and deserialization logic.

- **Files**:
  - `src/lib.rs` - Main library file exporting model structs and enums.
  - `src/patient_profile.rs` - Models related to patient profiles.
  - `src/problem.rs` - (Planned) Models related to problems.
  - `src/allergy.rs` - (Planned) Models related to allergies.

- **Key Components**:
  - **Patient Models**:
    - `Patient` struct - Represents a patient with detailed personal and medical information.
    - `PatientForCreate` struct - Data required to create a new patient.
    - Enums for categorical data:
      - `Sex`, `GenderIdentity`, `Race`, `Ethnicity`, `Pronouns`, etc.
    - Serialization with `serde` and custom date formats.
  - **Params Trait Implementation**:
    - `PatientQueryParams` struct - Used for querying patients.
    - Implements `Params` trait for use in client methods.

#### 1.4. `utils/`

- **Description**: The `utils` crate contains utility functions and helpers used across the SDK.

- **Files**:
  - `src/lib.rs` - Main library file exporting utility functions.
  - `src/time.rs` - Utilities related to time and date handling.

- **Key Components**:
  - **Time Utilities**:
    - Custom date and time formats using the `time` crate.
    - Helper functions for parsing and formatting dates.

---

### 2. `tests/`

- **Description**: Integration and unit tests for the SDK, ensuring correctness and reliability.

- **Files**:
  - `patient_service_tests.rs` - Tests for the `PatientService`.
  - (Planned) Other test files for additional services and components.

- **Key Components**:
  - **Testing Framework**:
    - Uses `tokio::test` for asynchronous tests.
    - Mocks HTTP requests using `httpmock`.
  - **Test Cases**:
    - Tests for successful API interactions.
    - Tests for error handling and edge cases.
    - Coverage for all methods in `PatientService`.

---

## Additional Notes

- **Error Handling**:
  - The SDK uses custom `Error` enums in both the `client` and `services` crates.
  - Errors are designed to be informative and to encapsulate underlying causes.

- **Logging**:
  - (Planned) Incorporation of logging for debugging and monitoring.

- **Documentation**:
  - All public structs, enums, and methods are documented using `///` comments.
  - Examples are provided where helpful.

- **Future Enhancements**:
  - Implementation of additional services (`ProblemService`, `AllergyService`, etc.).
  - Integration of continuous integration (CI) workflows.
  - Expansion of test coverage and addition of integration tests.
  - Preparation for publishing the SDK, including updating metadata and adding a README.

---

## File and Module Hierarchy

```
project-root/
├── Cargo.toml
├── libs/
│   ├── client/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── error.rs
│   │   │   ├── params.rs
│   │   │   └── response.rs
│   │   └── Cargo.toml
│   ├── services/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── error.rs
│   │   │   ├── patient_service.rs
│   │   │   └── problem_service.rs (planned)
│   │   ├── tests/
│   │   │   └── patient_service_tests.rs
│   │   └── Cargo.toml
│   ├── models/
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── patient_profile.rs
│   │   │   ├── problem.rs (planned)
│   │   │   └── allergy.rs (planned)
│   │   └── Cargo.toml
│   └── utils/
│       ├── src/
│       │   ├── lib.rs
│       │   └── time.rs
│       └── Cargo.toml
└── tests/
    └── (integration tests if any)

```

---

## Example Usage

```rust
use client::Client;
use services::PatientService;
use models::patient_profile::{PatientForCreate, Sex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = Client::new().await?;

    // Create a patient service instance
    let patient_service = PatientService::new(&client);

    // Create a new patient
    let new_patient = PatientForCreate {
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        dob: time::Date::from_calendar_date(1990, time::Month::January, 1)?,
        sex: Sex::Female,
        primary_physician: 123,
        caregiver_practice: 456,
        address: None,
        emails: None,
        insurances: vec![],
    };

    let patient = patient_service.create_patient(&new_patient).await?;

    println!("Created patient with ID: {}", patient.id);

    Ok(())
}
```

---

## Key Dependencies

- **`reqwest`**: HTTP client library for making requests.
- **`serde` and `serde_json`**: Serialization and deserialization of JSON data.
- **`serde_with`**: Extensions for `serde` to handle special cases.
- **`time`**: Date and time handling.
- **`derive_more`**: Simplifies the implementation of common traits.
- **`tokio`**: Asynchronous runtime for Rust.
- **`httpmock`** (dev-dependency): Mocking HTTP requests in tests.

---

## Testing

- **Unit Tests**: Located within each crate, focusing on individual units of code.
- **Integration Tests**: (Planned) Tests that cover interactions between components.
- **Mocking**: Uses `httpmock` to simulate API responses.
- **Running Tests**: Use `cargo test` to run all tests.

---

## Development Guidelines

- **Error Handling**: Propagate errors using the `Result` type and custom error enums.
- **Coding Standards**: Follow Rust best practices and conventions.
- **Documentation**: Document all public APIs and keep comments up to date.
- **Version Control**: Use Git for version control, with meaningful commit messages.
- **Continuous Integration**: (Planned) Set up CI pipelines for automated testing.

---

## Future Work

- **Implement Additional Services**: Expand the SDK to cover more API resources.
- **Enhance Error Handling**: Refine error messages and handling logic.
- **Add Logging**: Incorporate logging for better observability.
- **Publish the SDK**: Prepare for publishing on crates.io, including license and metadata.
- **Gather Feedback**: Solicit feedback to improve the SDK.

# Notes
