## Build tools & versions used

Rust 1.81.0
Postgresql 15
Docker 27.3.1

## Steps to run the app

Create a basic docker container and run migrations:

```bash
./scripts/init_db.sh
```

Launch cargo:

```bash
cargo run
```

You can now try with opening a browser on http://127.0.0.1:3000/nodes

## What was the reason for your focus? What problems were you trying to solve?

My focus was on developing a basic webapp with a well-structured database, because I believe it's important to create something that is practical and scalable, even within the constraints of a challenge. I prefer avoiding shortcuts or "quick and dirty" implementations that I wouldn't feel great deploying in production. The goal was to build something functional and reliable, even with limited time, that could be a foundation for real-world use.

To ensure maintainability and clarity, I set up GitHub Actions for automated workflows. This approach allows for clear, documented processes by codifying how the application is set up, tested, and run. I like to use code to document the project, as it not only proves that the project works but also serves as a living guide for future development or onboarding, while written documentation is great, they can easily get outdated with people forgetting to maintain it up to date.

## How long did you spend on this project?

~4hs

## Did you make any trade-offs for this project? What would you have done differently with more time?

Yes. With the limited time, I had to focus almost entirely on building a basic web app that could reliably get and display data. That meant I didn't have much time to add tests or proper logging. If I had more time, I’d definitely work on improving those areas to make the app more robust and easier to debug.

## What do you think is the weakest part of your project?

Tests. All the tests are sharing the same database, so if a test modifies the DB, it could cause data races. With more time, I’d set up a fresh database for each test to make sure they run independently and reliably.

## Is there any other information you’d like us to know?

If the docker doesn't work, check if you already have postgresql running (or something on the ports 5432)

Also, there is lots of questions about the requirement that isn't in the challenge

* Do you need the data be loaded on the startup? or should be a different endpoint
* Is it safe to assume the data will be always there?
* On conflict, should we just update the fields? or ignore it?
* etc.