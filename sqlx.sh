#!/bin/bash

sqlx migrate add create_subscriptions_table  

# add 
# CREATE TABLE subscriptions(
#    id uuid NOT NULL,
#    PRIMARY KEY (id),
#    email TEXT NOT NULL UNIQUE,
#    name TEXT NOT NULL,
#    subscribed_at timestamptz NOT NULL
# );


sqlx migrate run  # apply