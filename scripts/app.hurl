# Health check
GET http://127.0.0.1:8080/api/v1/health
HTTP 200

#------------------------------------------------------------------------------
# For app endpoint using admin user
#------------------------------------------------------------------------------

# login with admin user
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


# get user for user_id
GET http://127.0.0.1:8080/api/v1/admin/users
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$" count == 1
[Captures]
userid: jsonpath "$[0].id"


# FIXME: user logined at admin site, not app
# however, it works in app site with admin token
# roll is required in token claims


# add todo without token
POST http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos
Content-Type: application/json
{
    "title": "Programming",
    "description": "sample program using actix web",
    "status": "pending"
}
HTTP 401


# add todo
POST http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "title": "Programming",
    "description": "sample program using actix web",
    "status": "pending"
}
HTTP 200
[Asserts]
jsonpath "$.title" == "Programming"
jsonpath "$.description" == "sample program using actix web"
jsonpath "$.status" == "Pending" #FIXME: return `pending`


# get user todo list without token
GET http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos
HTTP 401


# get user todo list
GET http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].title" == "Programming"
jsonpath "$[0].description" == "sample program using actix web"
jsonpath "$[0].status" == "Pending" #FIXME: return `pending`
[Captures]
todoid: jsonpath "$[0].id"


# get user todo by todo_id without token
GET http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
HTTP 401


# get user todo by todo_id
GET http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$.title" == "Programming"
jsonpath "$.description" == "sample program using actix web"
jsonpath "$.status" == "Pending" #FIXME: return `pending`


# update todo without token
PUT http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Content-Type: application/json
{
    "title": "Programming2",
    "description": "sample program using actix web2",
    "status": "doing"
}
HTTP 401


# update todo without body
PUT http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{token}}
HTTP 400


# update todo
PUT http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "title": "Programming2",
    "description": "sample program using actix web2",
    "status": "doing"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
jsonpath "$.title" == "Programming2"
jsonpath "$.description" == "sample program using actix web2"
jsonpath "$.status" == "Doing" #FIXME: return `doing`


# partially update todo
PUT http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "title": "Programming3"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
jsonpath "$.title" == "Programming3"
jsonpath "$.description" == "sample program using actix web2"
jsonpath "$.status" == "Doing" #FIXME: return `doing`


# delete todo without token
DELETE http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
HTTP 401


# delete todo
DELETE http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{token}}
HTTP 204


#------------------------------------------------------------------------------
# operate app user todo by admin uesr
#------------------------------------------------------------------------------

# add app user
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


# add todo for new app user by admin user
POST http://127.0.0.1:8080/api/v1/app/users/{{app_userid}}/todos
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "title": "Reading book",
    "description": "read books",
    "status": "pending"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
[Captures]
app_todoid: jsonpath "$.id"


# get todo for new app user by admin user
GET http://127.0.0.1:8080/api/v1/app/users/{{app_userid}}/todos/{{app_todoid}}
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
jsonpath "$.title" == "Reading book"
jsonpath "$.description" == "read books"
jsonpath "$.status" == "Pending" #FIXME: return `pending`


# update todo for new app user by admin user
PUT http://127.0.0.1:8080/api/v1/app/users/{{app_userid}}/todos/{{app_todoid}}
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "title": "Reading book2",
    "description": "read books2",
    "status": "doing"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
jsonpath "$.title" == "Reading book2"
jsonpath "$.description" == "read books2"
jsonpath "$.status" == "Doing" #FIXME: return `doing`


# delete todo for new app user by admin user
DELETE http://127.0.0.1:8080/api/v1/app/users/{{app_userid}}/todos/{{app_todoid}}
Authorization: Bearer {{token}}
HTTP 204

GET http://127.0.0.1:8080/api/v1/app/users/{{app_userid}}/todos/{{app_todoid}}
Authorization: Bearer {{token}}
HTTP 404


#------------------------------------------------------------------------------
# For app endpoint using app user
#------------------------------------------------------------------------------
POST http://127.0.0.1:8080/api/v1/admin/users
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "first_name": "Foo2",
    "last_name": "Bar2",
    "email": "foo.bar2@example.com",
    "password": "password9999",
    "is_admin": false
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
[Captures]
app2_userid: jsonpath "$.id"


# login with app user
# Login success
POST http://127.0.0.1:8080/api/v1/app/login
Content-Type: application/json
{
    "email": "foo.bar2@example.com",
    "password": "password9999"
}
HTTP 200
[Asserts]
jsonpath "$.token" exists
[Captures]
app2_token: jsonpath "$.token"


# add todo for app user
POST http://127.0.0.1:8080/api/v1/app/users/{{app2_userid}}/todos
Authorization: Bearer {{app2_token}}
Content-Type: application/json
{
    "title": "Shopping",
    "description": "I wanna buy clothes",
    "status": "pending"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
[Captures]
app2_todoid: jsonpath "$.id"


# get todo
GET http://127.0.0.1:8080/api/v1/app/users/{{app2_userid}}/todos/{{app2_todoid}}
Authorization: Bearer {{app2_token}}
HTTP 200
[Asserts]
jsonpath "$.title" == "Shopping"
jsonpath "$.description" == "I wanna buy clothes"
jsonpath "$.status" == "Pending" #FIXME: return `pending`


# update todo
PUT http://127.0.0.1:8080/api/v1/app/users/{{app2_userid}}/todos/{{app2_todoid}}
Authorization: Bearer {{app2_token}}
Content-Type: application/json
{
    "title": "Shopping2",
    "description": "I wanna buy clothes2",
    "status": "doing"
}
HTTP 200
[Asserts]
jsonpath "$.id" exists
jsonpath "$.title" == "Shopping2"
jsonpath "$.description" == "I wanna buy clothes2"
jsonpath "$.status" == "Doing" #FIXME: return `doing`


# delete todo
DELETE http://127.0.0.1:8080/api/v1/app/users/{{app2_userid}}/todos/{{app2_todoid}}
Authorization: Bearer {{app2_token}}
HTTP 204


#------------------------------------------------------------------------------
# operate admin's todo by app user
#------------------------------------------------------------------------------

# add todo
POST http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos
Authorization: Bearer {{token}}
Content-Type: application/json
{
    "title": "Programming",
    "description": "sample program using actix web",
    "status": "pending"
}
HTTP 200
[Captures]
todoid: jsonpath "$.id"


# get todo list for admin user (other user)
GET http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos
Authorization: Bearer {{app2_token}}
HTTP 401


# get todo for admin user (other user)
GET http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{app2_token}}
HTTP 401


# update todo for admin user (other user)
PUT http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{app2_token}}
Content-Type: application/json
{
    "title": "Programming 2",
    "description": "sample program using actix web 2",
    "status": "doing"
}
HTTP 401


# delete todo
DELETE http://127.0.0.1:8080/api/v1/app/users/{{userid}}/todos/{{todoid}}
Authorization: Bearer {{app2_token}}
HTTP 401
