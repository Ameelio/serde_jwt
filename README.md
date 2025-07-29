# Serializer/Deserializer for JWT
This is a simple serde implementation to take the raw string, decode it from base64 and
allow deserialization and serialization. What this does not do is handle the signature creation
or verification.

## Reasons
- JWTs are used with very different claims and contexts.
- Its more secure to have the developer choose their crypto libraries and decide
  what fields in the header and claims are relevant to them.
- This mostly deals with the common headaches involving base64 decoding and parsing the initial
  input string.
