# ClipStash
## NOTE: DUE TO MASSIVE BREAKING CHANGES, I'VE ARCHIVED THIS PROJECT, EFFECTIVE FROM 20/10/2024.
ClipStash is the OG Web Service that allows users to paste and share clipboard content. Built on top of Rust.
![image](https://github.com/ishaaqziyan/ClipStash/assets/98882071/17db8e76-8dc3-4f95-8ffc-9f6c8cccf460)

To run the program👇
```
cargo run --bin httpd
```


Note:

The project requires additional steps in order to properly build. You will need the `sqlx-cli` tool which can be installed by running👇

```
cargo install sqlx-cli
```

After installing the tool, you can configure the database for the project by running👇

```
sqlx database setup
```

Addition Troubleshooting Steps:
If you are getting compilation errors related to `rocket::response::content::Html` or `rocket::response::content::RawHtml` then run these commands to fix:

```
cargo update --package rocket --precise 0.5.0-rc.1
cargo update --package rocket_codegen --precise 0.5.0-rc.1
cargo update --package rocket_http --precise 0.5.0-rc.1
```
