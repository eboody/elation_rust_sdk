The goal:

```rust
use services::PatientService;
use services::error::Error;

async fn example() -> Result<(), Error> {
    let client = Client::new().await?;
    let service = PatientService::new(&client);

    match service.get_patient(123).await {
        Ok(patient) => {
            println!("Patient: {:?}", patient);
            Ok(())
        }
        Err(Error::InvalidInput(msg)) => {
            eprintln!("Invalid input: {}", msg);
            Err(Error::InvalidInput(msg))
        }
        Err(Error::ServiceError(msg)) => {
            eprintln!("Service error: {}", msg);
            Err(Error::ServiceError(msg))
        }
        Err(Error::ClientError(client_error)) => {
            eprintln!("Client error: {}", client_error);
            Err(Error::ClientError(client_error))
        }
        Err(e) => {
            eprintln!("Other error: {}", e);
            Err(e)
        }
    }
}
```
