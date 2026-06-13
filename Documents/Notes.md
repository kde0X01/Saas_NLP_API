# Evolution of The Code

## Phase 1

Phase 1 is Based on:

- [x] Backend very Simple.

- [x] Frontend Wasm based on Yew.

The phase 1 code is the **Github** `main` branch.

## Phase 2

Phase 2 is where we gonna introduce new features:

- [x] Refactor API endpoints and add versioning:
  - [x] Implement path-based versioning as planned in `README.md` (e.g., `/api/v1/...`).
  - [x] Change the article fetching URI from `\api\table\date` to a more standard RESTful approach like `/api/v1/articles?year=2024&month=5&day=20`. This uses query parameters for filtering, which is more flexible and conventional.

- [x] Introducing SSL for HTTPS.

- [x] Authentication Oauth2 using `Google Auth`

- [x] test the SSL endpoint [test URL](https://127.0.0.1:8088/api/v1/articles?year=2021&month=1&day=12)

### Resources

[OpenID Spec](https://openid.net/specs/openid-connect-core-1_0.html)

# Phase 2

## Summary

Implemented authentication token handling and a user indicator for the Yew web client.

1. Created Auth Utility Module (utils.rs)
   - Functions to get/set tokens and user email from localStorage
   - is_authenticated() to check auth status

2. Updated Auth Callback (auth_callback.rs)
   - After storing the token, fetches user info from /api/v1/protected
   - Stores user email in localStorage
   - Redirects to dashboard after a brief delay
3. Updated Dashboard (dashbord.rs)
   - Includes Authorization: Bearer <token> header in API requests
   - Retrieves token from localStorage before making requests
   - Logs when token is missing

4. Created User Indicator Component (components.rs)
    - Fixed position indicator in the top-right corner
    - Shows logged-in user's email when authenticated
    - Shows "Login" link when not authenticated
    - Includes a Dashboard link when logged in
5. Added Layout Component (app.rs)
    - Wraps all pages with the user indicator
    - User indicator appears on all pages

Changes Made:
      + Added gloo-timers dependency for async delays
      + Added js-sys dependency
      + Updated web-sys features to include Window

The web client now:
    + Stores and retrieves JWT tokens from localStorage
    + Includes tokens in API requests to the protected /articles endpoint
    + Displays the logged-in user's email on all pages
    + Provides easy navigation to Dashboard when authenticated
    + All pages now show the user indicator, and the dashboard automatically includes the authentication token when fetching articles.
