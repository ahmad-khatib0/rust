
create database __ezytutor_web_ssr__;
create user __ssruser__ with password 'mypassword';
grant all privileges on database __ezytutor_web_ssr__ to __ssruser__;


drop table if exists ezyweb_user;

/* Create tables. */
/* Note: Don't put a comma after last field */

create table ezyweb_user(
    username varchar(20) primary key,
    tutor_id INT,    
    user_password CHAR(100) not null
);
