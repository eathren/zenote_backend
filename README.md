# Zenote Backend
Zenote is a collaborative graph note-based system designed to revolutionize the way teams collaborate on notes. By leveraging a graph-based data structure, Zenote enables users to create, manage, and visualize their notes in an interconnected environment, enhancing the retrieval and organization of information.

## Features
Collaborative Editing: Real-time collaboration across teams.
Graph-Based Notes: Organize notes in a flexible, non-linear fashion.
Permission Management: Fine-grained access control for users and organizations.
Scalable Architecture: Built on Rust and PostgreSQL for performance and reliability.
Getting Started
These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

## Prerequisites
Rust (latest stable version)
PostgreSQL
sqlx-cli for handling database migrations.
Setup
Clone the repository
```bash
Copy code
git clone https://yourrepository.com/zenote_backend.git
cd zenote_backend
```
Setup the Database
Ensure PostgreSQL is running, and create two databases: one for development and one for testing.

```bash
createdb zenote_db
createdb zenote_db_test
```
Environment Variables
Create a .env file in the root directory and update it with your database credentials:

```dotenv
POSTGRES_DB=zenote_db
POSTGRES_PASSWORD=yourpassword
DATABASE_URL=postgres://postgres:yourpassword@localhost/zenote_db

POSTGRES_DB_TEST=zenote_db_test
POSTGRES_PASSWORD_TEST=yourtestpassword
TEST_DATABASE_URL=postgres://postgres:yourtestpassword@localhost:5433/zenote_db_test
```

## Run Database Migrations

```bash
sqlx migrate run
Repeat for the test database by adjusting the DATABASE_URL temporarily or by using the test environment variable setup.
```
## Build and Run
```bash
Copy code
cargo run
```
Testing
Run the automated tests for this system with:

```bash
cargo test
```

## Contributing
Please read CONTRIBUTING.md for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning
We use SemVer for versioning. For the versions available, see the tags on this repository.

## License
This project is licensed under the GNU License - see the LICENSE.md file for details.

## Acknowledgments
Thanks to the Rust community for the invaluable resources.
Special thanks to all contributors and testers.


https://lucid.app/lucidchart/d1dc93ec-e4e8-4fd4-908a-0b87fa95b7cf/edit?viewport_loc=-888%2C-1227%2C4407%2C3462%2C0_0&invitationId=inv_ee2bc8e1-d456-4514-b7f3-d924c2418ed2
