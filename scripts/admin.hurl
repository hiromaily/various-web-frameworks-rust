# Health check
GET http://127.0.0.1:8080/api/v1/health
HTTP 200

# OpenAPI
GET http://127.0.0.1:8080/openapi.json
HTTP 200

#------------------------------------------------------------------------------
# For admin endpoint using admin user
#------------------------------------------------------------------------------

# Login with wrong password
# expects error
POST http://127.0.0.1:8080/api/v1/admin/login
Content-Type: application/json
{
    "email": "foobar@zmail.com",
    "password": "xxxxxxxxxxx"
}
HTTP 401


# Login success
POST http://127.0.0.1:8080/api/v1/admin/login
Content-Type: application/json
{
    "email": "john.doe@example.com",
    "password": "password1234"
}
HTTP 200
[Asserts]
jsonpath "$.token" exists
[Captures]
token: jsonpath "$.token"


# user list without token
GET http://127.0.0.1:8080/api/v1/admin/users
HTTP 401


# user list
GET http://127.0.0.1:8080/api/v1/admin/users
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$" count == 1
#jsonpath "$[0].id" == 1
jsonpath "$[0].first_name" == "John"
jsonpath "$[0].last_name" == "Doe"
jsonpath "$[0].email" == "john.doe@example.com"
jsonpath "$[0].password" exists
jsonpath "$[0].is_admin" == true
jsonpath "$[0].created_at" exists
[Captures]
userid: jsonpath "$[0].id"


# get user without token
GET http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
HTTP 401


# get uesr by wrong id
GET http://127.0.0.1:8080/api/v1/admin/users/99999
Authorization: Bearer {{token}}
HTTP 404


# get uesr by id
GET http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$.id" == {{userid}}
jsonpath "$.first_name" == "John"
jsonpath "$.last_name" == "Doe"
jsonpath "$.email" == "john.doe@example.com"
jsonpath "$.password" exists
jsonpath "$.is_admin" == true
jsonpath "$.created_at" exists


# update user without token
PUT http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
Content-Type: application/json
{
    "first_name": "John3",
    "last_name": "Doe3",
    "email": "john.doe3@example.com",
    "password": "password12345",
    "is_admin": true
}
HTTP 401


# update user without body
PUT http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
Authorization: Bearer {{token}}
HTTP 400


# FIXME: update user with wrong body
# PUT http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
# Authorization: Bearer {{token}}
# Content-Type: application/json
# {
#     "title": "Programming3"
# }
# HTTP 400


# update user
PUT http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "first_name": "John2",
    "last_name": "Doe2",
    "email": "john.doe2@example.com",
    "password": "password12345",
    "is_admin": true
}
HTTP 200
[Asserts]
jsonpath "$.id" exists


# partially update user
PUT http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "first_name": "John3",
    "last_name": "Doe3"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists


# make sure updated user by getting user
# get user by id
GET http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$.id" == {{userid}}
jsonpath "$.first_name" == "John3"
jsonpath "$.last_name" == "Doe3"
jsonpath "$.email" == "john.doe2@example.com"
jsonpath "$.password" exists
jsonpath "$.is_admin" == true
jsonpath "$.created_at" exists


# delete user without token
DELETE http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
HTTP 401


# delete user
DELETE http://127.0.0.1:8080/api/v1/admin/users/{{userid}}
Authorization: Bearer {{token}}
HTTP 204


# make sure no record by getting user list
GET http://127.0.0.1:8080/api/v1/admin/users
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$" count == 0


#------------------------------------------------------------------------------
# For admin endpoint using app user
#------------------------------------------------------------------------------

# add app user (not admin user)
POST http://127.0.0.1:8080/api/v1/admin/users
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "first_name": "Foo",
    "last_name": "Bar",
    "email": "foo.bar@example.com",
    "password": "password9999",
    "is_admin": false
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
[Captures]
app_userid: jsonpath "$.id"


# Login error because user is not admin user
POST http://127.0.0.1:8080/api/v1/admin/login
Content-Type: application/json
{
    "email": "foo.bar@example.com",
    "password": "password9999"
}
HTTP 401


# delete app user
DELETE http://127.0.0.1:8080/api/v1/admin/users/{{app_userid}}
Authorization: Bearer {{token}}
HTTP 204


#------------------------------------------------------------------------------
# reset
#------------------------------------------------------------------------------

# reset data by adding initial data
POST http://127.0.0.1:8080/api/v1/admin/users
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "password": "password1234",
    "is_admin": true
}
HTTP 200
