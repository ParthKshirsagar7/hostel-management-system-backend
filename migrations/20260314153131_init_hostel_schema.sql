-- Hostel model
CREATE TABLE hostels (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    -- Flattened Address
    address_line_1 VARCHAR(255) NOT NULL,
    address_line_2 VARCHAR(255),
    city VARCHAR(100) NOT NULL,
    pin_code VARCHAR(20) NOT NULL
);

-- Hostel room model
CREATE TABLE rooms (
    id SERIAL PRIMARY KEY,
    hostel_id INT NOT NULL REFERENCES hostels(id) ON DELETE CASCADE,
    room_name VARCHAR(50) NOT NULL,
    fee DECIMAL(10, 2) NOT NULL,
    deposit DECIMAL(10, 2) NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT TRUE,
    UNIQUE(hostel_id, room_name) -- A hostel can't have two "Room 101"s
);

-- Student model
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100),
    student_contact VARCHAR(20) NOT NULL,
    
    -- Flattened Address
    address_line_1 VARCHAR(255) NOT NULL,
    address_line_2 VARCHAR(255),
    city VARCHAR(100) NOT NULL,
    pin_code VARCHAR(20) NOT NULL,

    -- Parent/Guardian Info
    father_full_name VARCHAR(100),
    father_contact VARCHAR(20),
    father_email VARCHAR(100),
    
    mother_full_name VARCHAR(100),
    mother_contact VARCHAR(20),
    mother_email VARCHAR(100),
    
    guardian_full_name VARCHAR(100),
    guardian_contact VARCHAR(20),
    guardian_email VARCHAR(100),

    -- This is the Postgres equivalent of your application-level validation
    CONSTRAINT check_emergency_contact CHECK (
        father_contact IS NOT NULL OR 
        mother_contact IS NOT NULL OR 
        guardian_contact IS NOT NULL
    )
);

-- Allocation Model
CREATE TABLE allocations (
    id SERIAL PRIMARY KEY,
    student_id INT NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    room_id INT NOT NULL REFERENCES rooms(id) ON DELETE RESTRICT,
    check_in_date DATE NOT NULL DEFAULT CURRENT_DATE,
    check_out_date DATE
);

-- Payment Model
CREATE TABLE payments (
    id SERIAL PRIMARY KEY,
    student_id INT NOT NULL REFERENCES students(id) ON DELETE RESTRICT,
    hostel_id INT NOT NULL REFERENCES hostels(id) ON DELETE RESTRICT,
    
    -- amount can be positive (they paid us) or negative (we refunded them)
    amount DECIMAL(10, 2) NOT NULL, 
    
    -- payment_type can be room_fees, penalty, deposit, or refund
    payment_type VARCHAR(50) NOT NULL CHECK (payment_type IN ('room_fees', 'penalty', 'deposit', 'refund')),
    
    payment_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Crucial for refunds: "Refunded 40% for early exit on March 15"
    description TEXT 
);