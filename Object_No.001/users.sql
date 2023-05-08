create table if not exists `test_table`
(
`id` int unsigned  not null auto_increment primary key,
`username` varchar(256) not null ,
`info` varchar(256) not null ,
`group` int unsigned  not null
);
-- testing
