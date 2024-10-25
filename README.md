# Elation Health Rust SDK

A Rust SDK for interacting with the Elation Health API, supporting various services and data models related to patient data, medical records, scheduling, and more.

## Features

- **Modular Libraries**: Organized modules for `client`, `config`, `error`, `models`, `sdk`, `services`, and `utils`.
- **Comprehensive Models**: Structs and methods for various Elation Health data models, including `patient profiles`, `insurance`, `billing`, `scheduling`, and more.
- **Service Layer**: Functions for handling and managing patient records, lab orders, allergy documentation, and other key data.
- **Token Service**: Dedicated module for handling token-based authentication.

## Project Structure

```plaintext
.
├── Cargo.toml               # Main Cargo file
├── Dockerfile               # Dockerfile for containerization
├── libs/                    # Libraries for SDK components
│   ├── client               # Client library for Elation Health API requests
│   ├── config               # Configuration module
│   ├── debug_deserialize    # Helper module for debugging JSON deserialization errors
│   ├── error                # Error handling library
│   ├── models               # Data models for various Elation Health resources
│   ├── sdk                  # SDK interface for interacting with Elation Health
│   ├── services             # Service layer for business logic and API calls
│   └── utils                # Utility functions for environment management, encoding, etc.
└── token-service/           # Token-based authentication service
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- Docker (optional, for containerized deployments)

### Installation

Clone the repository and install dependencies:

```bash
git clone https://github.com/eboody/elation_rust_sdk.git
cd elation_rust_sdk
cargo build
```

### Configuration

To configure the SDK, edit the configuration file or use environment variables in `.env` format. The `config` module in `libs/config` handles these settings.

### Usage

#### Example: Patient Service

Here’s how to use the SDK to manage patient data:

```rust
use client::Client;
use services::patient_profile::{PatientService, PatientQueryParams};

#[tokio::main]
async fn main() {
    let client = Client::new().await.unwrap();
    let patient_service = PatientService::new(&client);

    // Retrieve a patient by ID
    let patient_id = 123456;
    match patient_service.get(patient_id).await {
        Ok(patient) => println!("Patient info: {:?}", patient),
        Err(err) => eprintln!("Error retrieving patient: {:?}", err),
    }

    // Search for patients by query parameters
    let query_params = PatientQueryParams {
        first_name: Some("John".to_string()),
        ..Default::default()
    };
    match patient_service.find(query_params).await {
        Ok(patients) => println!("Patients found: {:?}", patients.results),
        Err(err) => eprintln!("Error searching patients: {:?}", err),
    }
}
```

#### Example: Allergy Service

```rust
use client::Client;
use services::patient_profile::{AllergyService, AllergyForCreate};
use models::patient_profile::AllergyStatus;
use time::Date;

#[tokio::main]
async fn main() {
    let client = Client::new().await.unwrap();
    let allergy_service = AllergyService::new(&client);

    // Create a new allergy
    let new_allergy = AllergyForCreate {
        status: AllergyStatus::Active,
        start_date: Date::from_calendar_date(1980, time::Month::January, 1).unwrap(),
        reaction: Some("nausea and vomiting".to_string()),
        name: "Erythromycin".to_string(),
        patient: 64072843265,
        ..Default::default()
    };

    match allergy_service.post(&new_allergy).await {
        Ok(allergy) => println!("Created allergy: {:?}", allergy),
        Err(err) => eprintln!("Error creating allergy: {:?}", err),
    }
}
```

#### Token Service

The `token-service` module provides authentication using tokens. Start it independently as a service if needed:

```bash
cd token-service
cargo run
```

## Testing

The `libs/services/tests` directory contains unit tests for services like `patient_service`, `allergy_service`, etc. Run tests with:

```bash
cargo test
```

## Docker Deployment

To deploy the SDK as a container:

```bash
docker build -t elation_rust_sdk .
docker run -e API_KEY=your_api_key elation_rust_sdk
```

## License

This project is licensed under the MIT License.
