# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Subscriber model struct.`
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Subscriber repository.`
    -   [ ] Commit: `Implement list_all function in Subscriber repository.`
    -   [ ] Commit: `Implement delete function in Subscriber repository.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1

1. In Rust, using a trait for Subscriber would provide the flexibility to implement different types of subscribers with varying behaviors, adhering to the Observer pattern's principle of decoupling objects. However, in our case, BambangShop only requires a single type of subscriber with no varying behavior, a single model struct may suffice without the need for a trait.

2. Using a DashMap is necessary for this case because it allows for concurrent and thread-safe access while also ensuring the uniqueness of each id or url by using them as keys, providing efficient O(1) lookups, insertions, and deletions. A Vec does not enforce uniqueness and would require O(n) time complexity to search for items, which is less efficient for these operations.

3. The Singleton pattern ensures that a class has only one instance and provides a global point of access to it; however, it does not inherently guarantee thread safety. DashMap is a thread-safe, concurrent map that implements this principle internally, ensuring safe access from multiple threads without additional synchronization. Therefore, even when implementing a Singleton in Rust, using a thread-safe data structure like DashMap is necessary to maintain thread safety without manual synchronization mechanisms.

#### Reflection Publisher-2

1. Separating "Service" and "Repository" from a "Model" adheres to the Single Responsibility Principle, one of the SOLID principles of object-oriented design. The "Model" should focus on representing the domain data, the "Repository" handles data storage and retrieval, abstracting the persistence layer, and the "Service" contains business logic, acting as a bridge between the controllers and the repository. This separation facilitates maintainability, testing, and scaling as each layer has a clear responsibility and can be developed and modified independently.

2. If we only use the Model without distinct Service or Repository layers, each model would have to handle its own persistence, business logic, and possibly even presentation logic. This would lead to several issues, such as Models would become bloated with methods for CRUD operations, business rules, validation, and possibly UI logic, making them harder to understand and maintain. Therefore, By keeping models focused on representing domain data, and handling business logic and data access in separate layers, the system becomes more modular, maintainable, and adaptable to change.

3. Yes, I've previously delved into Postman before undertaking this task. From what I've seen, Postman's capabilities have been quite beneficial to me as a backend developer, particularly for API testing. Typically, I would employ Postman to validate the backend APIs that I developed (although initially, I utilized Insomnia) prior to their integration with separate frontend applications such as Next.js or Flutter. Similarly, for this job, I find Postman to be extremely effective in emulating interactions between the client and server.

#### Reflection Publisher-3

1. In this tutorial case, the Observer Pattern variation being used is the Push model. This is indicated by the NotificationService::notify function, which actively pushes notification data to the subscribers when an event occurs, such as the creation or deletion of a product. The subscribers are notified of the change with the relevant data being sent to them, rather than them having to request or pull the data from the publisher.

2. Using the Pull model, subscribers would control when to fetch updates, potentially conserving resources if updates are infrequent, but might face delays in receiving new information. Conversely, this approach could increase server load with frequent polling and complicate subscriber logic to manage update checks.

3. If multi-threading is not used in the notification process, notifications would be sent sequentially on a single thread, blocking the execution flow until each notification is processed. This could significantly slow down the response times for the publishing operation, especially if there are many subscribers or if the notification delivery involves time-consuming tasks like network communication.