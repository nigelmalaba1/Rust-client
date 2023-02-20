 ## Rust Client with Auth0 
 
 
 This client makes an HTTP requests an access token from Auth0 to access an API. 
 
 
 ## Overview
 
In this Rust project, I define two structs: AuthRequestBody and AuthResponseBody, which correspond to the request and response bodies of the OAuth 2.0 token request, respectively. I'm using the serde crate to serialize the request body into JSON, and to deserialize the response body from JSON.

I'm then creating a reqwest client, setting the appropriate URL and headers for the OAuth 2.0 token endpoint, and sending the AuthRequestBody as JSON in a POST request. If the response is successful, I'm printing the access token from the AuthResponseBody. Otherwise, I'm printing the status code of the response.
