# Notes from the code

## Old Code to keep

``` rust
// #[function_component(DashboardPage)]
// pub fn dashboard() -> Html {
//     //     log!("Yew client started");
//     let table_state: UseStateHandle<String> = use_state(|| "leseco".to_string()); // initial table name
//     let date_state: UseStateHandle<String> = use_state(|| "2021-01-15".to_string()); // Initial date for example
//     let data_state: UseStateHandle<Option<FinalData>> = use_state(|| None); // State to hold fetched data
//     let error_state: UseStateHandle<Option<String>> = use_state(|| None); // State to hold error messages (for both login and data fetch)
//     let loading_state: UseStateHandle<bool> = use_state(|| false); // State to indicate loading status

//     // New state for authentication
//     let username_state: UseStateHandle<String> = use_state(|| "".to_string());
//     let password_state: UseStateHandle<String> = use_state(|| "".to_string());
//     let token_state: UseStateHandle<Option<String>> = use_state(|| None); // Stores the JWT token

//     // Callback for table input change
//     // This callback updates the table state when the input changes
//     let on_table_change: Callback<Event> = Callback::from({
//         let table_state: UseStateHandle<String> = table_state.clone();
//         move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into::<web_sys::HtmlInputElement>(); // Get the input element
//             table_state.set(input.value()); // Update the table state with the input value
//         }
//     });

//     // Callback for date input change
//     let on_date_change: Callback<Event> = Callback::from({
//         let date_state: UseStateHandle<String> = date_state.clone();
//         move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into::<web_sys::HtmlInputElement>(); // Get the input element
//             date_state.set(input.value()); // Update the date state with the input value
//         }
//     });

//     // Callbacks for username and password input changes
//     let on_username_change: Callback<Event> = Callback::from({
//         let username_state: UseStateHandle<String> = username_state.clone();
//         move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
//             username_state.set(input.value());
//         }
//     });

//     let on_password_change: Callback<Event> = Callback::from({
//         let password_state: UseStateHandle<String> = password_state.clone();
//         move |e: Event| {
//             let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
//             password_state.set(input.value());
//         }
//     });

//     // Callback for login button click
//     let on_login_click: Callback<MouseEvent> = Callback::from({
//         let username_state: UseStateHandle<String> = username_state.clone();
//         let password_state: UseStateHandle<String> = password_state.clone();
//         let token_state: UseStateHandle<Option<String>> = token_state.clone();
//         let error_state: UseStateHandle<Option<String>> = error_state.clone();
//         let loading_state: UseStateHandle<bool> = loading_state.clone();

//         move |_| {
//             let username: String = (*username_state).clone();
//             let password: String = (*password_state).clone();
//             let token_state: UseStateHandle<Option<String>> = token_state.clone();
//             let error_state: UseStateHandle<Option<String>> = error_state.clone();
//             let loading_state: UseStateHandle<bool> = loading_state.clone();

//             loading_state.set(true);
//             error_state.set(None);
//             token_state.set(None); // Clear previous token

//             spawn_local(async move {
//                 let login_url: &'static str = "http://127.0.0.1:9090/login"; // Assuming /login is not under /api
//                 let creds: UserCredentials = UserCredentials { username, password };
//                 log!(format!("Attempting login to: {}", login_url));

//                 match reqwest::Client::new()
//                     .post(login_url)
//                     .json(&creds)
//                     .send()
//                     .await
//                 {
//                     Ok(response) => {
//                         if response.status().is_success() {
//                             match response.json::<AuthResponse>().await {
//                                 Ok(auth_data) => {
//                                     log!(format!(
//                                         "Login successful for user: {}",
//                                         auth_data.username
//                                     ));
//                                     token_state.set(Some(auth_data.token));
//                                 }
//                                 Err(e) => {
//                                     log!(format!("Failed to parse auth JSON: {:?}", e));
//                                     error_state.set(Some(format!(
//                                         "Login failed: Invalid response format: {}",
//                                         e
//                                     )));
//                                 }
//                             }
//                         } else {
//                             let status: reqwest::StatusCode = response.status();
//                             let text: String = response.text().await.unwrap_or_else(|_| {
//                                 "Could not read error response body".to_string()
//                             });
//                             log!(format!("Login API error: Status {} - {}", status, text));
//                             error_state.set(Some(format!("Login failed: {} - {}", status, text)));
//                         }
//                     }
//                     Err(e) => {
//                         log!(format!("Login request failed: {:?}", e));
//                         error_state.set(Some(format!("Login network error: {}", e)));
//                     }
//                 }
//                 loading_state.set(false);
//             });
//         }
//     });

//     // Callback for the button click to fetch data
//     // This callback triggers the API request when the button is clicked
//     // It uses `spawn_local` to run the async request without blocking the UI
//     let onclick: Callback<MouseEvent> = Callback::from({
//         let table_state: UseStateHandle<String> = table_state.clone();
//         let date_state: UseStateHandle<String> = date_state.clone();
//         let data_state: UseStateHandle<Option<FinalData>> = data_state.clone();
//         let error_state: UseStateHandle<Option<String>> = error_state.clone();
//         let loading_state: UseStateHandle<bool> = loading_state.clone();
//         let token_state: UseStateHandle<Option<String>> = token_state.clone(); // Capture token state

//         move |_| {
//             let table: String = (*table_state).clone();
//             let date: String = (*date_state).clone();
//             let data_state: UseStateHandle<Option<FinalData>> = data_state.clone();
//             let error_state: UseStateHandle<Option<String>> = error_state.clone();
//             let loading_state: UseStateHandle<bool> = loading_state.clone();
//             let token: Option<String> = (*token_state).clone(); // Get current token

//             if token.is_none() {
//                 error_state.set(Some("Please log in first to fetch data.".to_string()));
//                 return;
//             }
//             let token: String = token.unwrap(); // We know it's Some(String) now

//             loading_state.set(true);
//             error_state.set(None); // Clear previous errors
//             data_state.set(None); // Clear previous data

//             // Start the async request to fetch data from the API
//             log!(format!(
//                 "Fetching data for table: {}, date: {}",
//                 table, date
//             )); // This log is redundant with the one below
//             spawn_local(async move {
//                 let url = format!("http://127.0.0.1:9090/api/v1/{}/{}", table, date);
//                 log!(format!("Fetching from: {}", url));
//                 match reqwest::Client::new()
//                     .get(&url)
//                     .header("Authorization", format!("Bearer {}", token)) // Add Authorization header
//                     .send()
//                     .await
//                 {
//                     Ok(response) => {
//                         if response.status().is_success() {
//                             // Read the response body as text first for debugging
//                             let response_text = match response.text().await {
//                                 Ok(text) => text,
//                                 Err(e) => {
//                                     log!(format!("Failed to read response body as text: {:?}", e));
//                                     error_state
//                                         .set(Some(format!("Failed to read response body: {}", e)));
//                                     loading_state.set(false);
//                                     return; // Exit spawn_local block
//                                 }
//                             };
//                             log!(format!("Raw response body: {}", response_text));

//                             // Now try to parse the text as JSON
//                             match serde_json::from_str::<FinalData>(&response_text) {
//                                 Ok(data) => {
//                                     log!(format!("Data received: {:?}", data));
//                                     data_state.set(Some(data));
//                                 }
//                                 // If parsing fails, log the error and set the error state
//                                 Err(e) => {
//                                     log!(format!("Failed to parse JSON from text: {:?}", e));
//                                     // Include a snippet of the body in the error for easier debugging
//                                     let body_snippet =
//                                         response_text.chars().take(200).collect::<String>();
//                                     error_state.set(Some(format!(
//                                         "Failed to parse data: {}. Response snippet: '{}...'",
//                                         e, body_snippet
//                                     )));
//                                 }
//                             }
//                             // If parsing is successful, the data_state will be updated
//                         } else {
//                             let status: reqwest::StatusCode = response.status();
//                             let text: String = response.text().await.unwrap_or_else(|_| {
//                                 "Could not read error response body".to_string()
//                             });
//                             log!(format!("API error: Status {} - {}", status, text));
//                             error_state.set(Some(format!("API Error: {} - {}", status, text)));
//                         }
//                     }
//                     // If the request fails, log the error and set the error state
//                     Err(e) => {
//                         log!(format!("Request failed: {:?}", e));
//                         error_state.set(Some(format!("Network error: {}", e)));
//                     }
//                 }
//                 // Regardless of success or failure, set loading to false
//                 loading_state.set(false);
//             });
//         }
//     });

//     // Callback for logout button click
//     let on_logout_click: Callback<MouseEvent> = Callback::from({
//         let token_state: UseStateHandle<Option<String>> = token_state.clone();
//         let data_state: UseStateHandle<Option<FinalData>> = data_state.clone();
//         let error_state: UseStateHandle<Option<String>> = error_state.clone();
//         let username_state: UseStateHandle<String> = username_state.clone();
//         let password_state: UseStateHandle<String> = password_state.clone();

//         move |_| {
//             token_state.set(None); // Clear the token
//             data_state.set(None); // Clear any displayed data
//             error_state.set(None); // Clear any errors
//             username_state.set("".to_string()); // Clear username field
//             password_state.set("".to_string()); // Clear password field
//         }
//     });

//     // Render the HTML structure of the app
//     html! {
//         <> // Use a fragment to wrap multiple top-level elements
//             <div style="font-family: sans-serif; max-width: 800px; margin: 20px auto; padding: 20px; border: 1px solid #ccc; border-radius: 8px;">
//                 <h1>{"Rust API Client (Yew)"}</h1> // Moved this outside conditional rendering

//                 {
//                     if token_state.is_none() {
//                         html! {
//                             <div style="margin-top: 20px;">
//                                 <h2>{"Login"}</h2>
//                                 <div style="margin-bottom: 15px;">
//                                     <label for="username" style="display: block; margin-bottom: 5px;">{"Username:"}</label>
//                                     <input
//                                         id="username"
//                                         type="text"
//                                         value={(*username_state).clone()}
//                                         onchange={on_username_change}
//                                         style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
//                                     />
//                                 </div>
//                                 <div style="margin-bottom: 20px;">
//                                     <label for="password" style="display: block; margin-bottom: 5px;">{"Password:"}</label>
//                                     <input
//                                         id="password"
//                                         type="password"
//                                         value={(*password_state).clone()}
//                                         onchange={on_password_change}
//                                         style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
//                                     />
//                                 </div>
//                                 <button
//                                     onclick={on_login_click}
//                                     disabled={*loading_state}
//                                     style="padding: 10px 20px; background-color: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
//                                 >
//                                     { if *loading_state { "Logging in..." } else { "Login" } }
//                                 </button>
//                                 {
//                                     if error_state.is_some() {
//                                         html!{} // Error is displayed globally, no need to duplicate here
//                                     } else {
//                                         html! {
//                                             <p style="margin-top: 10px; color: #666;">{"Please log in to fetch data."}</p>
//                                         }
//                                     }
//                                 }

//                             </div>
//                         }
//                     } else {
//                         html! {
//                             <div style="margin-top: 20px;">
//                                 <h2>{"Logged In! Fetch News Data"}</h2>
//                                 <div style="margin-bottom: 15px;">
//                                     <label for="table_name" style="display: block; margin-bottom: 5px;">{"Table Name (e.g., leseco):"}</label>
//                                     <input
//                                         id="table_name"
//                                         type="text"
//                                         value={(*table_state).clone()}
//                                         onchange={on_table_change}
//                                         style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
//                                     />
//                                 </div>

//                                 <div style="margin-bottom: 20px;">
//                                     <label for="date_val" style="display: block; margin-bottom: 5px;">{"Date (YYYY-MM-DD):"}</label>
//                                     <input
//                                         id="date_val"
//                                         type="date"
//                                         value={(*date_state).clone()}
//                                         onchange={on_date_change}
//                                         style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;"
//                                     />
//                                 </div>

//                                 <button
//                                     onclick={onclick}
//                                     disabled={*loading_state}
//                                     style="padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
//                                 >
//                                     { if *loading_state { "Fetching..." } else { "Fetch Data" } }
//                                 </button>
//                                 <button
//                                     onclick={on_logout_click}
//                                     disabled={*loading_state}
//                                     style="margin-left: 10px; padding: 10px 20px; background-color: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
//                                 >
//                                     {"Logout"}
//                                 </button>
//                             </div>
//                         }
//                     }
//                 }

//                 {
//                     if let Some(data) = &*data_state {
//                         if data.data_results.is_empty() {
//                             html! {
//                                 <p style="margin-top: 20px; color: #555;">{"No articles found for the selected table and date."}</p>
//                             }
//                         } else {
//                             html! {
//                                 <div style="margin-top: 20px;">
//                                     <h2>{format!("Articles ({} found):", data.total_results)}</h2>

//                                     <table style="width: 100%; border-collapse: collapse; margin-top: 10px;" border_spacing=30px>
//                                         <thead>
//                                             <tr style="background-color: #f2f2f2;">
//                                                 <th> {"Date"}</th>
//                                                 <th>{"Category"}</th>
//                                                 <th>{"Title"}</th>
//                                                 <th>{"Link"}</th>
//                                                 // <th>{"Content"}</th>
//                                             </tr>
//                                         </thead>
//                                         <tbody>
//                                             { for data.data_results.iter().map(|article| html! {
//                                                 <tr>
//                                                     // <td>{article.pub_year}</td>
//                                                     // <td>{article.pub_month}</td>
//                                                     // <td>{article.pub_day}</td>
//                                                     <td>{article.pub_date()}</td>
//                                                     <td>{article.category.clone()}</td>
//                                                     <td>{article.title.clone()}</td>
//                                                     <td>
//                                                         <a href={article.link.clone()} target="_blank">{ "Link" }</a>
//                                                     </td>
//                                                     // <td>{article.content.clone()}</td>
//                                                 </tr>
//                                             }) }
//                                         </tbody>
//                                     </table>
//                                 </div>
//                             }
//                         }
//                     } else {
//                         html! {
//                             // Empty fragment or nothing if no data
//                         }
//                     }
//                 }
//             </div>

//             // Display errors regardless of login state
//             {
//                 if let Some(error) = &*error_state {
//                     html! {
//                         <p style="color: red; margin-top: 20px;">
//                             <strong>{"Error:"}</strong> {error}
//                         </p>
//                     }
//                 } else {
//                     html! {}
//                 }
//             }
//         </>
//     }
// }
```
