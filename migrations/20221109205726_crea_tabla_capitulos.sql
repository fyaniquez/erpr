-- capitulos clasifica productos por naturaleza
-- abarrotes, libreria, etc
CREATE TABLE IF NOT EXISTS public.capitulos
(
    id bigint NOT NULL,
    nombre TEXT NOT NULL UNIQUE,
    descripcion TEXT NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    CONSTRAINT capitulos_pkey PRIMARY KEY (id)
);

ALTER TABLE public.capitulos OWNER TO erp;
CREATE SEQUENCE public.capitulos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER TABLE public.capitulos_id_seq OWNER TO erp;
ALTER SEQUENCE public.capitulos_id_seq OWNED BY public.capitulos.id;
ALTER TABLE ONLY public.capitulos ALTER COLUMN id SET DEFAULT nextval('public.capitulos_id_seq'::regclass);
SELECT pg_catalog.setval('public.capitulos_id_seq', 24, true);

-- datos
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Abarrotes', '2021-03-21 17:09:45.106883', '2021-03-21 17:09:45.106883', 'Artículos alimenticios, como conservas alimenticias, especias y otros', 1);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Bazar', '2021-03-21 17:09:45.173733', '2021-03-21 17:09:45.173733', 'Artículos no alimenticios de consumo: pilas, focos', 2);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Bebidas', '2021-03-21 17:09:45.190238', '2021-03-21 17:09:45.190238', 'Líquidos para consumo personal', 3);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Carnes', '2021-03-21 17:09:45.206922', '2021-03-21 17:09:45.206922', 'Carne de diferentes animales', 4);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Comunicaciones', '2021-03-21 17:09:45.223642', '2021-03-21 17:09:45.223642', 'Artículos físicos y virtuales de comunicación', 5);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Cosméticos', '2021-03-21 17:09:45.240321', '2021-03-21 17:09:45.240321', 'Artículos de cuidado personal para todo género y edad', 6);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Electronicos', '2021-03-21 17:09:45.256865', '2021-03-21 17:09:45.256865', 'Audífonos, parlantes, memorias, etc', 7);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Embutidos', '2021-03-21 17:09:45.273572', '2021-03-21 17:09:45.273572', 'chorizos, salchichas, etc', 8);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Entretenimiento', '2021-03-21 17:09:45.290301', '2021-03-21 17:09:45.290301', 'Artículos de entretenimiento como juegos o videos', 9);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Ferreteria', '2021-03-21 17:09:45.30568', '2021-03-21 17:09:45.30568', 'Herramientas', 10);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Frutas', '2021-03-21 17:09:45.313699', '2021-03-21 17:09:45.313699', 'Frutas en general', 11);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Golosinas', '2021-03-21 17:09:45.322041', '2021-03-21 17:09:45.322041', 'Dulces, chicles, etc', 12);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Higiene', '2021-03-21 17:09:45.330318', '2021-03-21 17:09:45.330318', 'Productos de limpieza personal', 13);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Hogar', '2021-03-21 17:09:45.338705', '2021-03-21 17:09:45.338705', 'Objetos no alimenticios para uso en el hogar', 14);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Horneados', '2021-03-21 17:09:45.347056', '2021-03-21 17:09:45.347056', 'productos de pasteleria', 15);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Lácteos', '2021-03-21 17:09:45.355696', '2021-03-21 17:09:45.355696', 'Leche y derivados', 16);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Libreria', '2021-03-21 17:09:45.363932', '2021-03-21 17:09:45.363932', 'Articulos de escritorio', 17);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Licores', '2021-03-21 17:09:45.372361', '2021-03-21 17:09:45.372361', 'Bebidas alcoholicas', 18);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Limpieza', '2021-03-21 17:09:45.380379', '2021-03-21 17:09:45.380379', 'Artículos de limpieza de ambientes', 19);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Mascotas', '2021-03-21 17:09:45.401763', '2021-03-21 17:09:45.401763', 'Artículos alimenticios y no alimenticios para mascotas', 20);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Navidad', '2021-03-21 17:09:45.414146', '2021-03-21 17:09:45.414146', 'Artículos alimenticios y no alimenticios para navidad', 21);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Salud', '2021-03-21 17:09:45.422541', '2021-03-21 17:09:45.422541', 'Medicamentos', 22);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Servicios', '2021-03-21 17:09:45.430861', '2021-03-21 17:09:45.430861', 'Transcripciones, fotocopias, etc', 23);
INSERT INTO public.capitulos (nombre, created_at, updated_at, descripcion, id) VALUES ('Snacks', '2021-03-21 17:09:45.439291', '2021-03-21 17:09:45.439291', 'Snacks', 24);

-- constraints
-- trigger para preparar el insert
CREATE OR REPLACE FUNCTION capitulos_insert_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.created_at = now();
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER insert_prepara
    BEFORE INSERT ON capitulos
    FOR EACH ROW
    EXECUTE FUNCTION capitulos_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION capitulos_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON capitulos
    FOR EACH ROW
    EXECUTE FUNCTION capitulos_update_prepara();

