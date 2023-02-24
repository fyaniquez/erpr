-- configura la base de datos
-- configura extensión para generar uuid

CREATE USER erp WITH PASSWORD 'el10culalu';
ALTER DATABASE erprdb OWNER TO erp;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

