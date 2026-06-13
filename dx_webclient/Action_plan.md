# Webclient Action Plan

## Final Vision

The webclient will be a polished Dioxus single-page application for a news service with a SaaS-style user experience.
It will combine frontend navigation, authentication, protected content, and backend article loading into a cohesive portal.

Key capabilities:

- A responsive Home page that clearly explains the service and directs users to login or register.
- Login and registration screens tied to backend auth flow with Google sign-in support.
- A protected Dashboard page that only authenticated users can access.
- Structured routing via `dioxus-router` for `/`, `/login`, `/register`, `/dashboard`, `/faq`, `/offers`, and `/news`.
- Shared auth state via Dioxus signals/context with a client-side session model.
- Async news loading from the backend endpoint by date.
- Dedicated support pages for FAQ / Help, Offers / Prices, and News & Announcements.
- Reusable UI components for navigation, page layout, cards, forms, and article lists.

The final product will feel like a production-ready SaaS website: fast, easy to navigate, and secure.

---

## What success looks like

- The app boots with a working router and visible Home page.
- Users can navigate to login/register and back without refresh.
- Authentication state is shared globally and persists as long as the browser session remains active.
- The `/dashboard` route redirects unauthorized visitors to `/login`.
- News articles load from the backend and display in the dashboard or news page.
- Styling is consistent and the layout works on mobile and desktop.

---

## Action Plan

### 1. Define routes and page components

- Add `Route` enum for all application routes.
- Create page components for:
  - `HomePage`
  - `LoginPage`
  - `RegisterPage`
  - `DashboardPage`
  - `FaqPage`
  - `OffersPage`
  - `NewsPage`
- Mount `Router::<Route> {}` in `src/main.rs`.
- Confirm the router renders each page and navigation updates the URL.

### 2. Build shared layout and navigation

- Implement a reusable `Navbar` component.
- Add route-aware nav links and an auth state indicator.
- Create a basic layout wrapper component for common page structure.
- Use existing CSS assets in `assets/styling/` and `tailwind.css`.

### 3. Add auth state management

- Define a global `AuthState` signal or context type.
- Track `is_authenticated`, `user_name`, and `auth_token` (or session marker).
- Provide auth state to child components.
- Add login and logout functions that update the shared state.

### 4. Implement forms and protected route behavior

- Build a login form with both Google auth and direct submission UX hints.
- Build a registration form with fields such as email, password, and name.
- Add validation feedback for required fields.
- Protect `/dashboard` and any future private pages by redirecting unauthenticated users.

### 5. Connect backend data fetches

- Use `use_resource` (or `use_server_future` if SSR/hydration is needed) for article loading.
- Fetch from `https://127.0.0.1:8080/api/v1/articles?year={}&month={}&day={}`.
- Display articles in a list with title, date, summary, and link details.
- Add loading and error states to the UI.

### 6. Persist auth and handle sessions

- Choose a persistence mechanism for auth state: in-memory session, local storage, or cookies.
- Keep users logged in across reloads if tokens are available.
- Clear session on logout and return to Home or Login.

### 7. Polish UI and styles

- Apply consistent spacing, colors, and typography.
- Use cards for page content and article presentations.
- Ensure mobile-first responsiveness.
- Add hero sections, CTA buttons, and branded page headers.

### 8. Review, test, and refine

- Verify all routes render correctly.
- Test auth flows, route guarding, and state persistence.
- Validate backend requests and article rendering.
- Fix layout or styling issues and improve usability.

---

## Implementation notes

- Recommended files to add/update:
  - `src/main.rs`
  - `src/components/navbar.rs`
  - `src/views/home.rs`
  - `src/views/login.rs`
  - `src/views/register.rs`
  - `src/views/dashboard.rs`
  - `src/views/faq.rs`
  - `src/views/offers.rs`
  - `src/views/news.rs`
  - `src/components/auth.rs` or `src/state/auth.rs`
- Keep components small and reusable.
- Prefer Dioxus hooks for state and async data.

---

## Priority Tasks

1. Enable routing and page scaffolding.
2. Wire auth state and protected route logic.
3. Add login/register form flows.
4. Fetch news articles from the backend.
5. Polish UX and finalize page layout.
