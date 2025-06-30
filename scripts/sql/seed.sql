-- Seed data for schema public
DO $$
DECLARE
    user_id_val INTEGER;
BEGIN
    FOR i IN 1..10 LOOP
        INSERT INTO public.users (username, email) VALUES
            (random_string(15), random_email()) RETURNING id INTO user_id_val;

        INSERT INTO public.address (user_id, street, city, zip_code) VALUES
            (user_id_val, random_string(20) || ' St', random_string(10), LPAD((floor(random() * 99999) + 1)::text, 5, '0'));

        INSERT INTO public.employment_details (user_id, company_name, position, salary) VALUES
            (user_id_val, random_string(12) || ' Corp', random_string(10), (random() * 100000)::numeric(10, 2) + 30000);

        INSERT INTO public.investments (user_id, investment_type, amount, investment_date) VALUES
            (user_id_val, CASE floor(random() * 3) WHEN 0 THEN 'Stocks' WHEN 1 THEN 'Bonds' ELSE 'Real Estate' END, (random() * 50000)::numeric(10, 2) + 1000, random_date('2010-01-01'::DATE, '2023-12-31'::DATE));
    END LOOP;
END
$$ LANGUAGE plpgsql;

-- Seed data for schema tenant_1
DO $$
DECLARE
    user_id_val INTEGER;
BEGIN
    FOR i IN 1..10 LOOP
        INSERT INTO tenant_1.users (username, email) VALUES
            (random_string(15), random_email()) RETURNING id INTO user_id_val;

        INSERT INTO tenant_1.address (user_id, street, city, zip_code) VALUES
            (user_id_val, random_string(20) || ' Ave', random_string(10), LPAD((floor(random() * 99999) + 1)::text, 5, '0'));

        INSERT INTO tenant_1.employment_details (user_id, company_name, position, salary) VALUES
            (user_id_val, random_string(12) || ' Ltd', random_string(10), (random() * 100000)::numeric(10, 2) + 30000);

        INSERT INTO tenant_1.investments (user_id, investment_type, amount, investment_date) VALUES
            (user_id_val, CASE floor(random() * 3) WHEN 0 THEN 'Mutual Funds' WHEN 1 THEN 'Cryptocurrency' ELSE 'Commodities' END, (random() * 50000)::numeric(10, 2) + 1000, random_date('2010-01-01'::DATE, '2023-12-31'::DATE));
    END LOOP;
END
$$ LANGUAGE plpgsql;

-- Seed data for schema tenant_2
DO $$
DECLARE
    user_id_val INTEGER;
BEGIN
    FOR i IN 1..10 LOOP
        INSERT INTO tenant_2.users (username, email) VALUES
            (random_string(15), random_email()) RETURNING id INTO user_id_val;

        INSERT INTO tenant_2.address (user_id, street, city, zip_code) VALUES
            (user_id_val, random_string(20) || ' Rd', random_string(10), LPAD((floor(random() * 99999) + 1)::text, 5, '0'));

        INSERT INTO tenant_2.employment_details (user_id, company_name, position, salary) VALUES
            (user_id_val, random_string(12) || ' Inc', random_string(10), (random() * 100000)::numeric(10, 2) + 30000);

        INSERT INTO tenant_2.investments (user_id, investment_type, amount, investment_date) VALUES
            (user_id_val, CASE floor(random() * 3) WHEN 0 THEN 'ETFs' WHEN 1 THEN 'Real Estate' ELSE 'Retirement' END, (random() * 50000)::numeric(10, 2) + 1000, random_date('2010-01-01'::DATE, '2023-12-31'::DATE));
    END LOOP;
END
$$ LANGUAGE plpgsql;

-- Seed data for schema tenant_4
DO $$
DECLARE
    user_id_val INTEGER;
BEGIN
    FOR i IN 1..10 LOOP
        INSERT INTO tenant_4.users (username, email) VALUES
            (random_string(15), random_email()) RETURNING id INTO user_id_val;

        INSERT INTO tenant_4.address (user_id, street, city, zip_code) VALUES
            (user_id_val, random_string(20) || ' Blvd', random_string(10), LPAD((floor(random() * 99999) + 1)::text, 5, '0'));

        INSERT INTO tenant_4.employment_details (user_id, company_name, position, salary) VALUES
            (user_id_val, random_string(12) || ' LLC', random_string(10), (random() * 100000)::numeric(10, 2) + 30000);

        INSERT INTO tenant_4.investments (user_id, investment_type, amount, investment_date) VALUES
            (user_id_val, CASE floor(random() * 3) WHEN 0 THEN 'Bonds' WHEN 1 THEN 'Stocks' ELSE 'Forex' END, (random() * 50000)::numeric(10, 2) + 1000, random_date('2010-01-01'::DATE, '2023-12-31'::DATE));
    END LOOP;
END
$$ LANGUAGE plpgsql;

-- Seed data for schema public - new tables
DO $$
DECLARE
    user_id_val INTEGER;
    category_id_val INTEGER;
    product_id_val INTEGER;
    category_name TEXT;
    phone_types TEXT[] := ARRAY['Mobile', 'Home', 'Work'];
    destinations TEXT[] := ARRAY['Paris', 'London', 'New York', 'Tokyo', 'Rome', 'Dubai', 'Sydney', 'Rio de Janeiro'];
    category_names TEXT[] := ARRAY['Electronics', 'Books', 'Clothing', 'Home Goods', 'Sports'];
    product_names_electronics TEXT[] := ARRAY['Laptop', 'Smartphone', 'Headphones', 'Smartwatch'];
    product_names_books TEXT[] := ARRAY['Fiction Novel', 'Science Book', 'Cookbook', 'Biography'];
    product_names_clothing TEXT[] := ARRAY['T-Shirt', 'Jeans', 'Jacket', 'Shoes'];
    product_names_homegoods TEXT[] := ARRAY['Blender', 'Toaster', 'Coffee Maker', 'Vacuum Cleaner'];
    product_names_sports TEXT[] := ARRAY['Dumbbells', 'Yoga Mat', 'Running Shoes', 'Bicycle'];
BEGIN
    -- Insert initial categories for public schema
    FOREACH category_name IN ARRAY category_names
    LOOP
        INSERT INTO public.category_details (category_name, description) VALUES (category_name, 'Description for ' || category_name);
    END LOOP;

    FOR i IN 1..100 LOOP
        SELECT id FROM public.users ORDER BY random() LIMIT 1 INTO user_id_val;

        -- phone_numbers
        INSERT INTO public.phone_numbers (user_id, phone_number, type) VALUES
            (user_id_val, '+' || LPAD((floor(random() * 99999999999) + 1)::text, 11, '0'), phone_types[floor(random() * array_length(phone_types, 1)) + 1]);

        -- travel_plans
        INSERT INTO public.travel_plans (user_id, destination, start_date, end_date, budget) VALUES
            (user_id_val, destinations[floor(random() * array_length(destinations, 1)) + 1], random_date('2024-01-01'::DATE, '2025-12-31'::DATE), random_date('2026-01-01'::DATE, '2027-12-31'::DATE), (random() * 5000)::numeric(10, 2) + 500);

        -- product_catalog (requires category_details)
        SELECT id FROM public.category_details ORDER BY random() LIMIT 1 INTO category_id_val;
        IF category_id_val IS NOT NULL THEN
            INSERT INTO public.product_catalog (product_name, category_id, price, stock_quantity) VALUES
                (CASE (SELECT public.category_details.category_name FROM public.category_details WHERE id = category_id_val) WHEN 'Electronics' THEN product_names_electronics[floor(random() * array_length(product_names_electronics, 1)) + 1]
                                                                         WHEN 'Books' THEN product_names_books[floor(random() * array_length(product_names_books, 1)) + 1]
                                                                         WHEN 'Clothing' THEN product_names_clothing[floor(random() * array_length(product_names_clothing, 1)) + 1]
                                                                         WHEN 'Home Goods' THEN product_names_homegoods[floor(random() * array_length(product_names_homegoods, 1)) + 1]
                                                                         WHEN 'Sports' THEN product_names_sports[floor(random() * array_length(product_names_sports, 1)) + 1]
                                                                         ELSE random_string(15) END, 
                 category_id_val, (random() * 1000)::numeric(10, 2) + 10, (random() * 200)::integer + 1);
        END IF;

        -- order_details (requires product_catalog)
        SELECT id FROM public.product_catalog ORDER BY random() LIMIT 1 INTO product_id_val;
        IF product_id_val IS NOT NULL THEN
            INSERT INTO public.order_details (user_id, product_id, quantity) VALUES
                (user_id_val, product_id_val, (random() * 10)::integer + 1);
        END IF;
    END LOOP;
END
$$ LANGUAGE plpgsql;

-- Seed data for schema tenant_1 - new tables
DO $$
DECLARE
    user_id_val INTEGER;
    category_id_val INTEGER;
    product_id_val INTEGER;
    category_name TEXT;
    phone_types TEXT[] := ARRAY['Mobile', 'Home', 'Work'];
    destinations TEXT[] := ARRAY['Berlin', 'Madrid', 'Beijing', 'Cairo', 'Mexico City', 'Seoul', 'Moscow', 'Dublin'];
    category_names TEXT[] := ARRAY['Automotive', 'Health', 'Beauty', 'Kids', 'Pet Supplies'];
    product_names_automotive TEXT[] := ARRAY['Car Battery', 'Tire Pressure Gauge', 'Motor Oil', 'Wiper Blades'];
    product_names_health TEXT[] := ARRAY['Vitamins', 'Protein Powder', 'First Aid Kit', 'Thermometer'];
    product_names_beauty TEXT[] := ARRAY['Shampoo', 'Lotion', 'Perfume', 'Makeup Kit'];
    product_names_kids TEXT[] := ARRAY['Toy Car', 'Building Blocks', 'Doll', 'Stuffed Animal'];
    product_names_petsupplies TEXT[] := ARRAY['Dog Food', 'Cat Litter', 'Bird Cage', 'Fish Tank'];
BEGIN
    -- Insert initial categories for tenant_1 schema
    FOREACH category_name IN ARRAY category_names
    LOOP
        INSERT INTO tenant_1.category_details (category_name, description) VALUES (category_name, 'Description for ' || category_name);
    END LOOP;

    FOR i IN 1..100 LOOP
        SELECT id FROM tenant_1.users ORDER BY random() LIMIT 1 INTO user_id_val;

        -- phone_numbers
        INSERT INTO tenant_1.phone_numbers (user_id, phone_number, type) VALUES
            (user_id_val, '+' || LPAD((floor(random() * 99999999999) + 1)::text, 11, '0'), phone_types[floor(random() * array_length(phone_types, 1)) + 1]);

        -- travel_plans
        INSERT INTO tenant_1.travel_plans (user_id, destination, start_date, end_date, budget) VALUES
            (user_id_val, destinations[floor(random() * array_length(destinations, 1)) + 1], random_date('2024-01-01'::DATE, '2025-12-31'::DATE), random_date('2026-01-01'::DATE, '2027-12-31'::DATE), (random() * 5000)::numeric(10, 2) + 500);

        -- product_catalog (requires category_details)
        SELECT id FROM tenant_1.category_details ORDER BY random() LIMIT 1 INTO category_id_val;
        IF category_id_val IS NOT NULL THEN
            INSERT INTO tenant_1.product_catalog (product_name, category_id, price, stock_quantity) VALUES
                (CASE (SELECT tenant_1.category_details.category_name FROM tenant_1.category_details WHERE id = category_id_val) WHEN 'Automotive' THEN product_names_automotive[floor(random() * array_length(product_names_automotive, 1)) + 1]
                                                                            WHEN 'Health' THEN product_names_health[floor(random() * array_length(product_names_health, 1)) + 1]
                                                                            WHEN 'Beauty' THEN product_names_beauty[floor(random() * array_length(product_names_beauty, 1)) + 1]
                                                                            WHEN 'Kids' THEN product_names_kids[floor(random() * array_length(product_names_kids, 1)) + 1]
                                                                            WHEN 'Pet Supplies' THEN product_names_petsupplies[floor(random() * array_length(product_names_petsupplies, 1)) + 1]
                                                                            ELSE random_string(15) END, 
                 category_id_val, (random() * 1000)::numeric(10, 2) + 10, (random() * 200)::integer + 1);
        END IF;

        -- order_details (requires product_catalog)
        SELECT id FROM tenant_1.product_catalog ORDER BY random() LIMIT 1 INTO product_id_val;
        IF product_id_val IS NOT NULL THEN
            INSERT INTO tenant_1.order_details (user_id, product_id, quantity) VALUES
                (user_id_val, product_id_val, (random() * 10)::integer + 1);
        END IF;
    END LOOP;
END
$$ LANGUAGE plpgsql;

-- Seed data for schema tenant_2 - new tables
DO $$
DECLARE
    user_id_val INTEGER;
    category_id_val INTEGER;
    product_id_val INTEGER;
    category_name TEXT;
    phone_types TEXT[] := ARRAY['Mobile', 'Home', 'Work'];
    destinations TEXT[] := ARRAY['Singapore', 'Bangkok', 'Istanbul', 'Amsterdam', 'Prague', 'Vienna', 'Budapest', 'Lisbon'];
    category_names TEXT[] := ARRAY['Electronics', 'Books', 'Clothing', 'Home Goods', 'Sports'];
    product_names_electronics TEXT[] := ARRAY['Laptop', 'Smartphone', 'Headphones', 'Smartwatch'];
    product_names_books TEXT[] := ARRAY['Fiction Novel', 'Science Book', 'Cookbook', 'Biography'];
    product_names_clothing TEXT[] := ARRAY['T-Shirt', 'Jeans', 'Jacket', 'Shoes'];
    product_names_homegoods TEXT[] := ARRAY['Blender', 'Toaster', 'Coffee Maker', 'Vacuum Cleaner'];
    product_names_sports TEXT[] := ARRAY['Dumbbells', 'Yoga Mat', 'Running Shoes', 'Bicycle'];
BEGIN
    -- Insert initial categories for tenant_2 schema
    FOREACH category_name IN ARRAY category_names
    LOOP
        INSERT INTO tenant_2.category_details (category_name, description) VALUES (category_name, 'Description for ' || category_name);
    END LOOP;

    FOR i IN 1..100 LOOP
        SELECT id FROM tenant_2.users ORDER BY random() LIMIT 1 INTO user_id_val;

        -- phone_numbers
        INSERT INTO tenant_2.phone_numbers (user_id, phone_number, type) VALUES
            (user_id_val, '+' || LPAD((floor(random() * 99999999999) + 1)::text, 11, '0'), phone_types[floor(random() * array_length(phone_types, 1)) + 1]);

        -- travel_plans
        INSERT INTO tenant_2.travel_plans (user_id, destination, start_date, end_date, budget) VALUES
            (user_id_val, destinations[floor(random() * array_length(destinations, 1)) + 1], random_date('2024-01-01'::DATE, '2025-12-31'::DATE), random_date('2026-01-01'::DATE, '2027-12-31'::DATE), (random() * 5000)::numeric(10, 2) + 500);

        -- product_catalog (requires category_details)
        SELECT id FROM tenant_2.category_details ORDER BY random() LIMIT 1 INTO category_id_val;
        IF category_id_val IS NOT NULL THEN
            INSERT INTO tenant_2.product_catalog (product_name, category_id, price, stock_quantity) VALUES
                (CASE (SELECT tenant_2.category_details.category_name FROM tenant_2.category_details WHERE id = category_id_val) WHEN 'Electronics' THEN product_names_electronics[floor(random() * array_length(product_names_electronics, 1)) + 1]
                                                                         WHEN 'Books' THEN product_names_books[floor(random() * array_length(product_names_books, 1)) + 1]
                                                                         WHEN 'Clothing' THEN product_names_clothing[floor(random() * array_length(product_names_clothing, 1)) + 1]
                                                                         WHEN 'Home Goods' THEN product_names_homegoods[floor(random() * array_length(product_names_homegoods, 1)) + 1]
                                                                         WHEN 'Sports' THEN product_names_sports[floor(random() * array_length(product_names_sports, 1)) + 1]
                                                                         ELSE random_string(15) END, 
                 category_id_val, (random() * 1000)::numeric(10, 2) + 10, (random() * 200)::integer + 1);
        END IF;

        -- order_details (requires product_catalog)
        SELECT id FROM tenant_2.product_catalog ORDER BY random() LIMIT 1 INTO product_id_val;
        IF product_id_val IS NOT NULL THEN
            INSERT INTO tenant_2.order_details (user_id, product_id, quantity) VALUES
                (user_id_val, product_id_val, (random() * 10)::integer + 1);
        END IF;
    END LOOP;
END
$$ LANGUAGE plpgsql;

-- Seed data for schema tenant_4 - new tables
DO $$
DECLARE
    user_id_val INTEGER;
    category_id_val INTEGER;
    product_id_val INTEGER;
    category_name TEXT;
    phone_types TEXT[] := ARRAY['Mobile', 'Home', 'Work'];
    destinations TEXT[] := ARRAY['Cairo', 'Capetown', 'Nairobi', 'Marrakech', 'Accra', 'Lagos', 'Tunis', 'Dakar'];
    category_names TEXT[] := ARRAY['Food', 'Drinks', 'Office Supplies', 'Art Supplies', 'Musical Instruments'];
    product_names_food TEXT[] := ARRAY['Pasta', 'Rice', 'Cereal', 'Canned Goods'];
    product_names_drinks TEXT[] := ARRAY['Juice', 'Soda', 'Water Bottle', 'Coffee Beans'];
    product_names_officesupplies TEXT[] := ARRAY['Pens', 'Notebooks', 'Stapler', 'Printer Ink'];
    product_names_artsupplies TEXT[] := ARRAY['Paint Set', 'Brushes', 'Canvas', 'Sketchbook'];
    product_names_musicalinstruments TEXT[] := ARRAY['Guitar', 'Keyboard', 'Drum Set', 'Flute'];
BEGIN
    -- Insert initial categories for tenant_4 schema
    FOREACH category_name IN ARRAY category_names
    LOOP
        INSERT INTO tenant_4.category_details (category_name, description) VALUES (category_name, 'Description for ' || category_name);
    END LOOP;

    FOR i IN 1..100 LOOP
        SELECT id FROM tenant_4.users ORDER BY random() LIMIT 1 INTO user_id_val;

        -- phone_numbers
        INSERT INTO tenant_4.phone_numbers (user_id, phone_number, type) VALUES
            (user_id_val, '+' || LPAD((floor(random() * 99999999999) + 1)::text, 11, '0'), phone_types[floor(random() * array_length(phone_types, 1)) + 1]);

        -- travel_plans
        INSERT INTO tenant_4.travel_plans (user_id, destination, start_date, end_date, budget) VALUES
            (user_id_val, destinations[floor(random() * array_length(destinations, 1)) + 1], random_date('2024-01-01'::DATE, '2025-12-31'::DATE), random_date('2026-01-01'::DATE, '2027-12-31'::DATE), (random() * 5000)::numeric(10, 2) + 500);

        -- product_catalog (requires category_details)
        SELECT id FROM tenant_4.category_details ORDER BY random() LIMIT 1 INTO category_id_val;
        IF category_id_val IS NOT NULL THEN
            INSERT INTO tenant_4.product_catalog (product_name, category_id, price, stock_quantity) VALUES
                (CASE (SELECT tenant_4.category_details.category_name FROM tenant_4.category_details WHERE id = category_id_val) WHEN 'Food' THEN product_names_food[floor(random() * array_length(product_names_food, 1)) + 1]
                                                                      WHEN 'Drinks' THEN product_names_drinks[floor(random() * array_length(product_names_drinks, 1)) + 1]
                                                                      WHEN 'Office Supplies' THEN product_names_officesupplies[floor(random() * array_length(product_names_officesupplies, 1)) + 1]
                                                                      WHEN 'Art Supplies' THEN product_names_artsupplies[floor(random() * array_length(product_names_artsupplies, 1)) + 1]
                                                                      WHEN 'Musical Instruments' THEN product_names_musicalinstruments[floor(random() * array_length(product_names_musicalinstruments, 1)) + 1]
                                                                      ELSE random_string(15) END, 
                 category_id_val, (random() * 1000)::numeric(10, 2) + 10, (random() * 200)::integer + 1);
        END IF;

        -- order_details (requires product_catalog)
        SELECT id FROM tenant_4.product_catalog ORDER BY random() LIMIT 1 INTO product_id_val;
        IF product_id_val IS NOT NULL THEN
            INSERT INTO tenant_4.order_details (user_id, product_id, quantity) VALUES
                (user_id_val, product_id_val, (random() * 10)::integer + 1);
        END IF;
    END LOOP;
END
$$ LANGUAGE plpgsql; 