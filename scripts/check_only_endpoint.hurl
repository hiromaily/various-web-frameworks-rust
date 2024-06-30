GET http://127.0.0.1:8080/api/v1/health
HTTP 200

#------------------------------------------------------------------------------
# For admin
#------------------------------------------------------------------------------

POST http://127.0.0.1:8080/api/v1/admin/login
Content-Type: application/json
{
    "email": "john.doe@example.com",
    "password": "password1234"
}
HTTP 200

POST http://127.0.0.1:8080/api/v1/admin/users
Content-Type: application/json
{
    "first_name": "Foo",
    "last_name": "Bar",
    "email": "foo.bar@example.com",
    "password": "password9999",
    "is_admin": false
}
HTTP 200

GET http://127.0.0.1:8080/api/v1/admin/users
HTTP 200

GET http://127.0.0.1:8080/api/v1/admin/users/1
HTTP 200

PUT http://127.0.0.1:8080/api/v1/admin/users/1
Content-Type: application/json
{
    "first_name": "John2",
    "last_name": "Doe2",
    "email": "john.doe2@example.com",
    "password": "password12345",
    "is_admin": true
}
HTTP 200

DELETE http://127.0.0.1:8080/api/v1/admin/users/1
HTTP 204


#------------------------------------------------------------------------------
# For app
#------------------------------------------------------------------------------

POST http://127.0.0.1:8080/api/v1/app/login
Content-Type: application/json
{
    "email": "foo.bar2@example.com",
    "password": "password9999"
}
HTTP 200

POST http://127.0.0.1:8080/api/v1/app/users/1/todos
Content-Type: application/json
{
    "title": "Shopping",
    "description": "I wanna buy clothes",
    "status": "pending"
}
HTTP 200

GET http://127.0.0.1:8080/api/v1/app/users/1/todos
HTTP 200

GET http://127.0.0.1:8080/api/v1/app/users/1/todos/1
HTTP 200

PUT http://127.0.0.1:8080/api/v1/app/users/1/todos/1
Content-Type: application/json
{
    "title": "Shopping2",
    "description": "I wanna buy clothes2",
    "status": "doing"
}
HTTP 200

DELETE http://127.0.0.1:8080/api/v1/app/users/1/todos/1
HTTP 204
