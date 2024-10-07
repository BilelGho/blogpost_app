# BlogPost App

## Description
BlogPost App is a simple web application that allows users to create and delete blog posts. Users can also view a list of all blog posts.

## Features
- Create new blog posts
- View a list of all blog posts

## Installation 
### Docker: Recommended
1. Clone the repository
2. Build the app and deploy it in a docker container
    ```
    docker build -t blogpost_app  .
    ```
3. Start running the server inside the docker
    ```
    docker run blogpost_app -p <host_port>:8000
    ```
    `host_port` being the port to be used by the app
4. Access the web app using the browser on the URI:
    ```
    localhost:<host_port>/home
    ```

### Locally
#### Requirements
You should install Rust, Cargo and sqlx-cli.

sqlx-cli can be installed using Cargo:

```
cargo install sqlx-cli
```


#### Set up the database
1. Create a new file called `.env` with the content:
```
DATABASE_URL=sqlite://<database_path>
```
Replace `database_path` with the desired path to store the database in.

2. Create the database 
```
sqlx database create

```

3. Set up the database table
```
sqlx migrate run
```
#### Build the app
Either locally using:
   ``` 
   cargo run
   ```
or add the binary to the cargo path
```
cargo install path -- .
```
 and run 

```
./blogpost_app
```

#### 