CREATE SCHEMA IF NOT EXISTS public;
CREATE SCHEMA IF NOT EXISTS tenant_1;
CREATE SCHEMA IF NOT EXISTS tenant_2;
CREATE SCHEMA IF NOT EXISTS tenant_4;

-- Function to generate random string
CREATE OR REPLACE FUNCTION random_string(length INTEGER) RETURNS TEXT AS $$
SELECT array_to_string(array(
    SELECT substring('0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz' FROM (floor(random() * 62) + 1)::int FOR 1)
    FROM generate_series(1, length)
), '');
$$ LANGUAGE SQL VOLATILE;

-- Function to generate random email
CREATE OR REPLACE FUNCTION random_email() RETURNS TEXT AS $$
SELECT random_string(10) || '@example.com';
$$ LANGUAGE SQL VOLATILE;

-- Function to generate random date
CREATE OR REPLACE FUNCTION random_date(start_date DATE, end_date DATE) RETURNS DATE AS $$
SELECT (start_date + (random() * (end_date - start_date + 1))::integer)::DATE;
$$ LANGUAGE SQL VOLATILE;

-- Table definitions for schema public
CREATE TABLE IF NOT EXISTS public.users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS public.address (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES public.users(id),
    street VARCHAR(255),
    city VARCHAR(255),
    zip_code VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS public.employment_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES public.users(id),
    company_name VARCHAR(255),
    position VARCHAR(255),
    salary DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS public.investments (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES public.users(id),
    investment_type VARCHAR(255),
    amount DECIMAL(10, 2),
    investment_date DATE
);

CREATE TABLE IF NOT EXISTS public.phone_numbers (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES public.users(id),
    phone_number VARCHAR(20) NOT NULL,
    type VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS public.travel_plans (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES public.users(id),
    destination VARCHAR(255) NOT NULL,
    start_date DATE,
    end_date DATE,
    budget DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS public.category_details (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS public.product_catalog (
    id SERIAL PRIMARY KEY,
    product_name VARCHAR(255) NOT NULL,
    category_id INTEGER REFERENCES public.category_details(id),
    price DECIMAL(10, 2),
    stock_quantity INTEGER
);

CREATE TABLE IF NOT EXISTS public.order_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES public.users(id),
    product_id INTEGER NOT NULL REFERENCES public.product_catalog(id),
    quantity INTEGER,
    order_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table definitions for schema tenant_1
CREATE TABLE IF NOT EXISTS tenant_1.users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tenant_1.address (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_1.users(id),
    street VARCHAR(255),
    city VARCHAR(255),
    zip_code VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS tenant_1.employment_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_1.users(id),
    company_name VARCHAR(255),
    position VARCHAR(255),
    salary DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS tenant_1.investments (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_1.users(id),
    investment_type VARCHAR(255),
    amount DECIMAL(10, 2),
    investment_date DATE
);

CREATE TABLE IF NOT EXISTS tenant_1.phone_numbers (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_1.users(id),
    phone_number VARCHAR(20) NOT NULL,
    type VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS tenant_1.travel_plans (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_1.users(id),
    destination VARCHAR(255) NOT NULL,
    start_date DATE,
    end_date DATE,
    budget DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS tenant_1.category_details (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS tenant_1.product_catalog (
    id SERIAL PRIMARY KEY,
    product_name VARCHAR(255) NOT NULL,
    category_id INTEGER REFERENCES tenant_1.category_details(id),
    price DECIMAL(10, 2),
    stock_quantity INTEGER
);

CREATE TABLE IF NOT EXISTS tenant_1.order_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_1.users(id),
    product_id INTEGER NOT NULL REFERENCES tenant_1.product_catalog(id),
    quantity INTEGER,
    order_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table definitions for schema tenant_2
CREATE TABLE IF NOT EXISTS tenant_2.users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tenant_2.address (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_2.users(id),
    street VARCHAR(255),
    city VARCHAR(255),
    zip_code VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS tenant_2.employment_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_2.users(id),
    company_name VARCHAR(255),
    position VARCHAR(255),
    salary DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS tenant_2.investments (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_2.users(id),
    investment_type VARCHAR(255),
    amount DECIMAL(10, 2),
    investment_date DATE
);

CREATE TABLE IF NOT EXISTS tenant_2.phone_numbers (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_2.users(id),
    phone_number VARCHAR(20) NOT NULL,
    type VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS tenant_2.travel_plans (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_2.users(id),
    destination VARCHAR(255) NOT NULL,
    start_date DATE,
    end_date DATE,
    budget DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS tenant_2.category_details (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS tenant_2.product_catalog (
    id SERIAL PRIMARY KEY,
    product_name VARCHAR(255) NOT NULL,
    category_id INTEGER REFERENCES tenant_2.category_details(id),
    price DECIMAL(10, 2),
    stock_quantity INTEGER
);

CREATE TABLE IF NOT EXISTS tenant_2.order_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_2.users(id),
    product_id INTEGER NOT NULL REFERENCES tenant_2.product_catalog(id),
    quantity INTEGER,
    order_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table definitions for schema tenant_4
CREATE TABLE IF NOT EXISTS tenant_4.users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tenant_4.address (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_4.users(id),
    street VARCHAR(255),
    city VARCHAR(255),
    zip_code VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS tenant_4.employment_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_4.users(id),
    company_name VARCHAR(255),
    position VARCHAR(255),
    salary DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS tenant_4.investments (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_4.users(id),
    investment_type VARCHAR(255),
    amount DECIMAL(10, 2),
    investment_date DATE
);

CREATE TABLE IF NOT EXISTS tenant_4.phone_numbers (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_4.users(id),
    phone_number VARCHAR(20) NOT NULL,
    type VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS tenant_4.travel_plans (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_4.users(id),
    destination VARCHAR(255) NOT NULL,
    start_date DATE,
    end_date DATE,
    budget DECIMAL(10, 2)
);

CREATE TABLE IF NOT EXISTS tenant_4.category_details (
    id SERIAL PRIMARY KEY,
    category_name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE IF NOT EXISTS tenant_4.product_catalog (
    id SERIAL PRIMARY KEY,
    product_name VARCHAR(255) NOT NULL,
    category_id INTEGER REFERENCES tenant_4.category_details(id),
    price DECIMAL(10, 2),
    stock_quantity INTEGER
);

CREATE TABLE IF NOT EXISTS tenant_4.order_details (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES tenant_4.users(id),
    product_id INTEGER NOT NULL REFERENCES tenant_4.product_catalog(id),
    quantity INTEGER,
    order_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
); 