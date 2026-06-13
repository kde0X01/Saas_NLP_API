# Basic Information

- **Call Back url**: `https://127.0.0.1:8080/api/v1/protected`
- **Login url**: `https://127.0.0.1:8080/api/v1/auth/google`
- **Retieve Data**: `https://127.0.0.1:8080/api/v1/articles?year={}&month={}&day={}`

## Pages

- HomePage
- LoginPage
- registerPage
- DashboardPage (Protected Route)
- FAQ / Help
- Offers / Prices
- News & Annoncements

### Page Organisation

``` mermaid
flowchart TD

    HOME[HomePage]

    HOME --> LOGIN[Login]
    HOME --> REGISTER[Register]

    LOGIN --> AUTH[Authentication System]

    AUTH -->|Success| DASH[DashboardPage]
    AUTH -->|Failure| HOME

    REGISTER --> REGSYS[Registration System]

    REGSYS -->|Success| AUTH
    REGSYS -->|Failure| HOME

    style HOME fill:#dbeafe
    style AUTH fill:#ede9fe
    style REGSYS fill:#fef3c7
    style DASH fill:#dcfce7

```

For the `Dioxus` webapp client, this flow maps cleanly to route-based navigation:

``` bash
/ → HomePage
/login → Authentication page/component
/register → Registration page/component
/dashboard → Protected authenticated route
```

The flow itself is already well-structured for a Dioxus client application, especially because it cleanly separates:

- navigation
- authentication
- registration
- protected routes

That separation will help a lot once you start adding:

- auth state management
- JWT/session persistence
- route guards
- async API calls
- global context/signals

```
HomePage
   │
   ├── LoginPage
   ├── RegisterPage
   │
AuthState / Context
   │
ProtectedRoute
   │
DashboardPage
```

Which maps naturally to:

- dioxus-router
- shared auth context/signals
- Axum backend APIs
- token storage/cookies
