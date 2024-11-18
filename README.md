> [!IMPORTANT]
> Planned ALPHA Release date: 7 FEBRUARY 2025 <br>

> [!NOTE]  
> This Project is a work in progress and is not suitable to be used at this moment.<br>
> Star the repository for progress and preview release updates.

# Axum Super Forms
The Axum Super forms is light weight, feature packed, blazingly fast forms service that serves forms and collect form data from users. Designed with simplicity and reliablity in mind, It is aimed to be a self hostable monolithic service that runs on a single server leveraging fully asynchronous multi-threading combined with Redis cache and Postgres Database to deliver sub-millisecond response times even under high loads.

## To Build
1. Clone the repository on your machine.
```bash
git clone https://github.com/Mahinkumar/axum_super_forms.git
cd axum_super_forms
```
2. Make sure you have installed docker, Redis, Cargo(Rust) and NPM(Node).
3. Create a .env file with required environment variables or pass in variables when running based on the .env.example file.
4. Install NPM packages for Tailwind
```bash
npm install
```
5. Build and run the server using the following commands
```bash
cargo build --release #If you only want to build, use this command
cargo run --release #You can directly build and run with this single command
```
> [!NOTE]  
> The Server will automatically check for Existing DataBase entries and perform migrations if necessary.<br>
> It also create few default entries for testing and evaluation purposes.<br>
> Existing entries will not be overwritten (except for default entries) in case of conflicting.



## Features
1. Blazingly fast, Scales on demand.
2. Includes a built in Load Balancer.
3. Admin console with realtime analytics
4. Group policies to provide fine grained access to users
5. Authentication support for admins and users
6. Fully customizable forms
7. Dynamic form inputs (Updates in realtime)
8. Form history and record support for users and admin
9. Site customization support
10. Email Support
11. Form Previews
12. Feature packed UI with built-in Themes
13. Dark and Light modes by default
14. Responsive across all devices

## Architecture
<img width="700" alt="Axum Super Forms Architecture" src="https://github.com/user-attachments/assets/f2026e3c-75e4-42cf-adf9-7faadc9d17a0">

 
