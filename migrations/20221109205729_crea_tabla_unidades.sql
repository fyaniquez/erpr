-- unidades, tabla de 
-- abreviacion PAI
CREATE TABLE IF NOT EXISTS public.unidades
(
    id bigint NOT NULL,
    sigla TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    CONSTRAINT unidades_pkey PRIMARY KEY (id)
);

ALTER TABLE public.unidades OWNER TO erp;
CREATE SEQUENCE public.unidades_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER TABLE public.unidades_id_seq OWNER TO erp;
ALTER SEQUENCE public.unidades_id_seq OWNED BY public.unidades.id;
ALTER TABLE ONLY public.unidades ALTER COLUMN id SET DEFAULT nextval('public.unidades_id_seq'::regclass);

-- datos
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (1, 'bdj.', 'Bandeja', '2021-03-21 17:03:31.642288', '2021-03-21 17:03:31.642288');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (2, 'bar.', 'Barra', '2021-03-21 17:03:31.706206', '2021-03-21 17:03:31.706206');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (3, 'bid.', 'Bidón', '2021-03-21 17:03:31.722769', '2021-03-21 17:03:31.722769');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (4, 'Bs.', 'Bolivianos', '2021-03-21 17:03:31.739424', '2021-03-21 17:03:31.739424');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (5, 'bol.', 'Bolsa', '2021-03-21 17:03:31.756123', '2021-03-21 17:03:31.756123');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (6, 'bot.', 'Botella', '2021-03-21 17:03:31.772736', '2021-03-21 17:03:31.772736');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (7, 'caja', 'Caja', '2021-03-21 17:03:31.78938', '2021-03-21 17:03:31.78938');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (8, 'h100', 'Hasta 100', '2021-03-21 17:03:31.806439', '2021-03-21 17:03:31.806439');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (9, 'h30', 'Hasta 30', '2021-03-21 17:03:31.81296', '2021-03-21 17:03:31.81296');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (10, 'hoj.', 'Hoja', '2021-03-21 17:03:31.821778', '2021-03-21 17:03:31.821778');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (11, 'kg.', 'Kilogramo', '2021-03-21 17:03:31.829635', '2021-03-21 17:03:31.829635');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (12, 'lata', 'Lata', '2021-03-21 17:03:31.83814', '2021-03-21 17:03:31.83814');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (13, 'lb.', 'Libra', '2021-03-21 17:03:31.86207', '2021-03-21 17:03:31.86207');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (14, 'Lt.', 'Litro', '2021-03-21 17:03:31.881198', '2021-03-21 17:03:31.881198');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (15, 'paq.', 'Paquete', '2021-03-21 17:03:31.897797', '2021-03-21 17:03:31.897797');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (16, 'par', 'Par', '2021-03-21 17:03:31.914586', '2021-03-21 17:03:31.914586');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (17, 'pza.', 'Pieza', '2021-03-21 17:03:31.931203', '2021-03-21 17:03:31.931203');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (18, 'sach', 'Sachet', '2021-03-21 17:03:31.947788', '2021-03-21 17:03:31.947788');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (19, 'ser.', 'Servicio', '2021-03-21 17:03:31.963765', '2021-03-21 17:03:31.963765');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (20, 'sob.', 'Sobre', '2021-03-21 17:03:31.971563', '2021-03-21 17:03:31.971563');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (21, 'uni.', 'Unidad', '2021-03-21 17:03:31.97964', '2021-03-21 17:03:31.97964');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (22, 'vaso', 'Vaso', '2021-03-21 17:03:31.987934', '2021-03-21 17:03:31.987934');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (23, 'lote', 'Lote', '2021-04-25 19:45:34.052797', '2021-04-25 19:45:34.052797');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (24, 'rollo', 'Rollo', '2021-04-25 19:45:50.546433', '2021-04-25 19:45:50.546433');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (25, 'tarro', 'Tarro', '2021-04-25 19:49:44.00255', '2021-04-25 19:53:13.027365');
INSERT INTO public.unidades (id, sigla, nombre, created_at, updated_at) VALUES (27, 'cua.', 'cuadernillo', '2021-10-04 12:00:00', '2021-10-04 12:00:00');
-- ajusta numero de ultimo registro
SELECT pg_catalog.setval('public.unidades_id_seq', 27, true);

-- constraints
-- trigger para preparar el insert
CREATE OR REPLACE FUNCTION unidades_insert_prepara() 
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
    BEFORE INSERT ON unidades
    FOR EACH ROW
    EXECUTE FUNCTION unidades_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION unidades_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON unidades
    FOR EACH ROW
    EXECUTE FUNCTION unidades_update_prepara();

