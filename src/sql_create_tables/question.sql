CREATE TABLE question (
                          id INT NOT NULL primary key ,
                          question VARCHAR(255) NOT NULL,
                          poll_id INT NOT NULL,
                          foreign key (poll_id) references poll(id)
)