-- fabricas, tabla de
-- abreviación FAB

CREATE TABLE IF NOT EXISTS public.fabricas (
    id bigint NOT NULL,
    nombre TEXT NOT NULL,
    pais_id bigint NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL,
    CONSTRAINT fab_pkey PRIMARY KEY (id)
);


ALTER TABLE public.fabricas OWNER TO erp;

CREATE SEQUENCE public.fabricas_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.fabricas_id_seq OWNER TO erp;

ALTER SEQUENCE public.fabricas_id_seq OWNED BY public.fabricas.id;

ALTER TABLE ONLY public.fabricas ALTER COLUMN id SET DEFAULT nextval('public.fabricas_id_seq'::regclass);

-- datos

INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (48, 'Chips Ahoy', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (49, 'Chiriguano', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (50, 'Chocman', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (51, 'Chocolike', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (52, 'Cif', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (53, 'Cocky', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (54, 'Cofler', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (55, 'Colgate', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (56, 'Colonia Japonesa', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (58, 'Confort', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (59, 'Copacabana', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (60, 'Costa', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (61, 'Crick', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (62, 'Criollo', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (63, 'crismelos', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (64, 'Crisol', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (65, 'CureBand', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (66, 'Del Campo', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (68, 'Derby', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (69, 'Vita', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (70, 'Docile', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (71, 'Don Maximo', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (73, 'Doña Gusta', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (74, 'Dove', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (75, 'Downy', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (76, 'Duryea', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (77, 'El Ceibo', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (78, 'El Chasqui', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (79, 'El Faro', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (80, 'Elite', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (81, 'Eno', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (82, 'ENTEL', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (83, 'Estracto', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (84, 'Excelsior', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (85, 'Faber Castell', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (86, 'Fernando', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (87, 'Field', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (88, 'Fiesta', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (89, 'Industrias de Aceite', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (90, 'Fixer', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (91, 'Flavicold', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (92, 'Industrias Venado', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (93, 'Fresh Runy', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (94, 'Fruna', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (95, 'Frutigel', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (96, 'Garoto', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (97, 'Gatty', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (99, 'Gloria', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (100, 'Gomes da Costa', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (101, 'Dos en Uno', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (72, 'Alicorp S.A.A', 14, '2021-03-21 16:01:14.119001', '2021-04-15 01:16:14.531692');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (57, 'Fabrica de Chocolates y Dulces Condor', 4, '2021-03-21 16:01:14.119001', '2021-04-16 01:22:51.463108');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (24, 'Tisue', 14, '2021-03-21 16:01:14.119001', '2021-04-17 02:14:13.706357');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (25, 'Labach', 6, '2021-03-21 16:01:14.119001', '2021-04-19 13:00:50.75346');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (98, 'Industrias Alimenticias Fagal', 4, '2021-03-21 16:01:14.119001', '2021-04-19 14:03:05.845884');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (102, 'Guabira', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (103, 'Zel', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (40, 'La Cascada Ltda.', 4, '2021-03-21 16:01:14.119001', '2021-10-01 19:43:44.750471');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (67, 'Delizia', 4, '2021-03-21 16:01:14.119001', '2021-10-03 17:46:01.610403');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (104, 'Head & Shoulders', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (105, 'HP', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (106, 'Huawei', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (107, 'Huggies', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (108, 'INCO', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (109, 'Iris', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (110, 'Jhonnie Walker', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (111, 'Karina', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (112, 'Kevin', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (113, 'King', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (114, 'Kingston', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (115, 'Klin', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (116, 'Unilever A.', 2, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (117, 'Kohlberg', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (118, 'Korr', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (120, 'Soalpro', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (121, 'L.Ch', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (122, 'L&M', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (123, 'La Belgica', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (124, 'La Estrella', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (125, 'La Francesa', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (126, 'La Suprema', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (127, 'EBA', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (129, 'LGQ', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (130, 'Lider', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (131, 'Corporación Alimento Marino', 6, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (132, 'Limppano', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (133, 'Lix', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (134, 'Lombarda', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (135, 'Lux', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (136, 'Maggi', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (137, 'Maira', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (138, 'Manu', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (139, 'Maribel', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (140, 'Maruchan', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (141, 'Marutex', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (142, 'Maver', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (143, 'MegaUlix', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (144, 'Inti', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (145, 'Merheje', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (146, 'Merletto', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (147, 'Misioneros', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (148, 'Miura', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (149, 'Mogul', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (150, 'PIL', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (151, 'Monami', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (152, 'Mundial', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (153, 'Nabisco', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (154, 'Nivea', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (155, 'Nonmedical', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (157, 'Nucita', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (161, 'Oreo', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (162, 'Orieta', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (163, 'ORO', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (164, 'Palmolive', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (166, 'Pegafan', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (167, 'Pepsodent', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (169, 'Phelix', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (170, 'Philips', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (171, 'Phowermax', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (172, 'Piposal', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (173, 'Piter', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (174, 'Plus', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (175, 'Plusbelle', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (176, 'Sofia', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (177, 'Pond''s', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (178, 'PRAN', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (179, 'Princesa', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (180, 'Princo', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (181, 'Protex', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (182, 'Q''Rico', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (185, 'Regia', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (186, 'Rexona', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (187, 'Cosas Ricas', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (188, 'Rossi', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (189, 'Ruby''s', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (190, 'S.Riicc', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (191, 'Sabonis', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (192, 'CONUMAP', 10, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (193, 'Safra', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (194, 'Salvietti', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (195, 'Samsung', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (197, 'San Luis', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (198, 'Scott', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (199, 'Sedal', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (200, 'Hleks', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (201, 'Sibarita', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (202, 'Siglo de Oro', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (203, 'Skip', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (204, 'Banco Sol', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (165, 'Intradevco Industral', 14, '2021-03-21 16:01:14.119001', '2021-03-25 01:50:41.881985');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (160, 'Unilever Andina Bolivia', 4, '2021-03-21 16:01:14.119001', '2021-03-25 01:54:59.603951');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (158, 'Astrix S.A.', 4, '2021-03-21 16:01:14.119001', '2021-03-31 01:54:10.271602');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (128, 'INPASTAS S.A.', 4, '2021-03-21 16:01:14.119001', '2021-04-15 01:09:27.825122');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (159, 'Swedish Match Do Brasil S.A.', 5, '2021-03-21 16:01:14.119001', '2021-04-16 01:13:43.034763');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (196, 'Conservera San Lucas', 14, '2021-03-21 16:01:14.119001', '2021-04-19 13:07:27.662017');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (183, 'Energizer Brands LLC', 5, '2021-03-21 16:01:14.119001', '2021-04-19 20:06:28.1066');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (205, 'Tusequis', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (207, 'Super Glue', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (168, 'Papelera Vinto', 4, '2021-03-21 16:01:14.119001', '2021-09-20 18:32:43.651481');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (184, 'Bebidas Bolivianas BBO S.A.', 4, '2021-03-21 16:01:14.119001', '2021-09-23 14:40:28.28126');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (209, 'Sylvania', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (210, 'Tapsin', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (211, 'Tasty', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (212, 'Terruño', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (213, 'TIGER', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (214, 'TIGO', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (217, 'Trigal', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (218, 'Tron', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (219, 'Tropical', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (220, 'UHU', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (222, 'Van Camp''s', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (223, 'VIVA', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (224, 'Wanabym', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (225, 'Windsor', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (119, 'Kimberly Clark Perú', 14, '2021-03-21 16:01:14.119001', '2021-03-25 01:41:21.12066');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (227, 'Algodones Aconcagua', 2, '2021-03-25 01:41:45.299581', '2021-03-25 01:41:45.299581');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (228, 'RLED', 7, '2021-03-30 21:39:39.899536', '2021-03-30 21:39:39.899536');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (215, 'Pepsico', 5, '2021-03-21 16:01:14.119001', '2021-04-01 01:37:59.414582');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (33, 'Poplar Capital S.A', 4, '2021-03-21 16:01:14.119001', '2021-04-13 01:13:18.604298');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (229, 'Fabrica Cruceña de Levaduras S.A.', 4, '2021-04-13 01:26:56.712753', '2021-04-13 01:26:56.712753');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (230, 'Led Bulb', 7, '2021-04-15 00:54:49.031747', '2021-04-15 00:54:49.031747');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (23, 'Baby Ñoño', 7, '2021-03-21 16:01:14.119001', '2021-04-17 02:10:12.296923');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (1, 'Feng Ruen', 7, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (2, 'Sin Nombre', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (3, 'Cervecería Boliviana Nacional', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (4, 'Aconcagua Foods', 2, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (5, 'Act II', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (6, 'Adams', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (7, 'CARSA', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (8, 'Aguai', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (9, 'Ajinomoto', 11, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (10, 'Alcos', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (11, 'Amazonas', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (13, 'Anapqui', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (14, 'GUSTOSSI', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (15, 'Coca Cola Company', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (16, 'Arcadia', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (17, 'ArcoIris', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (18, 'ARCOR', 2, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (19, 'Industrias Falco', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (20, 'Asatex', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (21, 'Aval', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (22, 'Axe', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (26, 'Bayer', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (27, 'Belen', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (28, 'Bolivar', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (29, 'BomBomBum', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (30, 'Bombril', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (31, 'Breik', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (32, 'BRIO', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (34, 'Bunge Alimentos', 14, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (35, 'Calesita', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (36, 'Canel', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (37, 'Caranavi', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (38, 'Carozzi', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (39, 'Casa Real', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (41, 'Caserita', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (42, 'Purina', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (43, 'Nestle', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (44, 'CERENAT', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (45, 'Hansa', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (46, 'Chawei', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (47, 'Cheff Gusto', 4, '2021-03-21 16:01:14.119001', '2021-03-21 16:01:14.119001');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (231, 'Fabrica de ceras', 4, '2021-04-19 13:04:33.285187', '2021-04-19 13:04:33.285187');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (226, 'Santiago Saenz S.A', 2, '2021-03-21 16:01:14.119001', '2021-04-19 18:36:38.158306');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (232, 'Yone', 7, '2021-09-13 22:29:55.579908', '2021-09-13 22:29:55.579911');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (12, 'Ambrosoli', 2, '2021-03-21 16:01:14.119001', '2021-09-16 23:50:07.523501');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (236, 'TIO', 7, '2021-09-17 14:32:54.716386', '2021-09-17 14:32:54.716389');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (237, 'Madison', 7, '2021-09-17 17:49:11.743294', '2021-09-17 17:49:11.743296');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (206, 'Copelme', 4, '2021-03-21 16:01:14.119001', '2021-09-19 16:08:11.448327');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (216, 'Toscano', 4, '2021-03-21 16:01:14.119001', '2021-09-20 19:14:51.831661');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (239, 'Toskano', 4, '2021-09-20 19:25:27.47723', '2021-09-20 19:25:27.477233');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (240, 'Lider', 14, '2021-10-03 20:19:45.422968', '2021-10-03 20:19:45.42297');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (241, 'La Tablada', 4, '2021-10-04 18:46:51.704394', '2021-10-04 18:46:51.704396');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (242, 'Skill hands', 7, '2021-10-04 20:11:03.929515', '2021-10-04 20:11:03.929518');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (243, 'Inmapel', 4, '2021-10-04 20:25:17.207176', '2021-10-04 20:25:17.207179');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (244, 'Bombillas del sur', 4, '2021-10-05 16:22:26.43303', '2021-10-05 16:22:26.433032');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (245, 'Stabilo', 1, '2021-10-05 18:52:27.448308', '2021-10-05 18:52:27.448311');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (246, 'Procter & Gamble', 11, '2021-10-05 19:17:30.217276', '2021-10-05 19:17:30.217278');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (247, 'Paper King S.R.L.', 4, '2021-10-05 20:57:08.152024', '2021-10-05 20:57:08.152026');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (248, 'Canadios', 4, '2021-10-05 21:10:05.905104', '2021-10-05 21:10:05.905107');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (249, 'Luri', 4, '2021-10-05 21:35:21.458427', '2021-10-05 21:35:21.458429');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (250, 'Acrilex Tintas', 5, '2021-10-05 21:41:54.564549', '2021-10-05 21:41:54.564552');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (251, 'Plasticolor', 7, '2021-10-05 22:01:04.814045', '2021-10-05 22:01:04.814047');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (252, 'Tio LED', 7, '2021-10-05 22:06:47.561865', '2021-10-05 22:06:47.561867');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (253, 'Playing Cards', 7, '2021-10-05 22:25:52.962796', '2021-10-05 22:26:36.182801');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (254, 'Harman International', 11, '2021-10-05 23:23:43.136217', '2021-10-05 23:23:43.136219');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (255, 'Monopol', 4, '2021-10-05 23:55:03.704394', '2021-10-05 23:55:03.704397');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (256, 'Mezgo', 4, '2021-10-06 00:05:28.84907', '2021-10-06 00:13:23.705351');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (233, 'Marko', 7, '2021-09-13 23:34:16.282817', '2021-10-06 22:53:45.923596');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (257, 'Zuixua', 7, '2021-10-06 23:18:24.936529', '2021-10-06 23:18:24.936532');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (258, 'Acricolor', 4, '2021-10-11 21:05:58.842391', '2021-10-11 21:05:58.842393');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (259, 'Feverhand Faber', 1, '2021-10-11 22:20:08.187611', '2021-10-11 22:20:08.187614');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (260, 'Full Color', 4, '2021-10-11 22:57:13.696504', '2021-10-11 22:57:13.696507');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (261, 'Sony', 16, '2021-10-11 23:48:46.734593', '2021-10-11 23:48:46.734596');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (262, 'Penguin', 7, '2021-10-16 23:33:34.446833', '2021-10-16 23:33:34.446835');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (263, 'Top Line', 14, '2021-10-16 23:58:07.569358', '2021-10-16 23:58:07.56936');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (264, 'WILMS GMBH', 1, '2021-10-22 21:49:44.572341', '2021-10-22 21:49:44.572344');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (265, 'The Castell', 4, '2021-10-22 23:18:20.774235', '2021-10-22 23:18:20.774239');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (266, 'Industrias europeas SAC', 14, '2021-10-23 23:25:28.447847', '2021-10-23 23:25:28.447853');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (267, 'Lentelux', 7, '2021-10-25 13:50:38.41556', '2021-10-25 13:50:38.415565');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (268, 'Crysanthemum', 7, '2021-10-25 14:33:48.21202', '2021-10-25 14:33:48.212023');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (269, 'Great Hall', 7, '2021-10-25 14:48:08.707028', '2021-10-25 14:48:08.707031');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (270, 'Pelikan', 1, '2021-10-25 15:45:12.646572', '2021-10-25 15:45:12.646575');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (271, 'Great Wall', 7, '2021-10-26 22:34:32.164463', '2021-10-26 22:34:32.164467');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (272, 'Laubsageblatter', 1, '2021-10-26 22:42:47.264906', '2021-10-26 22:43:33.441535');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (273, 'ABC', 4, '2021-10-26 23:31:13.172176', '2021-10-26 23:31:13.172179');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (274, 'Mereje', 5, '2021-11-01 01:05:41.224519', '2021-11-01 01:05:41.224525');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (275, 'XZV', 2, '2021-11-01 01:10:30.733631', '2021-11-01 01:10:30.733634');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (276, 'Hair Grips', 7, '2021-11-01 01:24:59.135436', '2021-11-01 01:24:59.135439');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (277, 'Sea Gull', 7, '2021-11-01 01:36:39.82735', '2021-11-01 01:36:39.827352');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (278, 'Theoto', 5, '2021-11-01 01:50:23.349093', '2021-11-01 01:50:23.349096');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (279, 'Five Fingers', 4, '2021-11-01 02:11:07.88904', '2021-11-01 02:11:07.88906');
INSERT INTO public.fabricas (id, nombre, pais_id, created_at, updated_at) VALUES (280, 'Milcar', 7, '2021-11-03 23:27:07.28559', '2021-11-03 23:27:07.285592');

-- ajusta ultimo registro
SELECT pg_catalog.setval('public.fabricas_id_seq', 280, true);

-- constraints
CREATE INDEX fab_idx_on_pais_id ON public.fabricas USING btree (pais_id);
ALTER TABLE ONLY public.fabricas
    ADD CONSTRAINT fab_fk_pais FOREIGN KEY (pais_id) REFERENCES public.paises(id);

ALTER TABLE public.fabricas
    ADD CONSTRAINT fab_uk_paisid_nombre UNIQUE (pais_id, nombre);

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION fabricas_insert_prepara() 
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
    BEFORE INSERT ON fabricas
    FOR EACH ROW
    EXECUTE FUNCTION fabricas_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION fabricas_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON fabricas
    FOR EACH ROW
    EXECUTE FUNCTION fabricas_update_prepara();
