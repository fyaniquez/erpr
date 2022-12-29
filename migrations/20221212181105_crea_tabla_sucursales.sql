--
-- PostgreSQL database dump
--

-- Dumped from database version 13.9 (Debian 13.9-0+deb11u1)
-- Dumped by pg_dump version 13.9 (Debian 13.9-0+deb11u1)

-- Started on 2022-12-21 18:11:05 -04


CREATE TABLE public.sucursales (
    id bigint NOT NULL,
    nombre TEXT NOT NULL,
    empresa_id bigint NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.sucursales OWNER TO erp;

--
-- TOC entry 3240 (class 0 OID 0)
-- Dependencies: 260
-- Name: COLUMN sucursales.catalogo_id; Type: COMMENT; Schema: public; Owner: erp
--

CREATE SEQUENCE public.sucursales_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.sucursales_id_seq OWNER TO erp;

--
-- TOC entry 3241 (class 0 OID 0)
-- Dependencies: 261
-- Name: sucursales_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: erp
--

ALTER SEQUENCE public.sucursales_id_seq OWNED BY public.sucursales.id;


--
-- TOC entry 3099 (class 2604 OID 16690)
-- Name: sucursales id; Type: DEFAULT; Schema: public; Owner: erp
--

ALTER TABLE ONLY public.sucursales ALTER COLUMN id SET DEFAULT nextval('public.sucursales_id_seq'::regclass);


--
-- TOC entry 3233 (class 0 OID 16623)
-- Dependencies: 260
-- Data for Name: sucursales; Type: TABLE DATA; Schema: public; Owner: erp
--

INSERT INTO public.sucursales (id, nombre, empresa_id, created_at, updated_at) VALUES (1, 'Casa Matriz', 1, '2021-03-21 22:08:48.141185', '2021-03-21 22:08:48.141185');


--
-- TOC entry 3242 (class 0 OID 0)
-- Dependencies: 261
-- Name: sucursales_id_seq; Type: SEQUENCE SET; Schema: public; Owner: erp
--

SELECT pg_catalog.setval('public.sucursales_id_seq', 1, true);


-- constraints
ALTER TABLE ONLY public.sucursales
    ADD CONSTRAINT sucursales_pkey PRIMARY KEY (id);

CREATE INDEX index_sucursales_on_empresa_id 
    ON public.sucursales USING btree (empresa_id);
ALTER TABLE ONLY public.sucursales
    ADD CONSTRAINT fk_suc_emp FOREIGN KEY (empresa_id) 
        REFERENCES public.empresas(id);

ALTER TABLE public.sucursales
    ADD CONSTRAINT uk_suc_nombre UNIQUE (empresa_id, nombre);

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION sucursales_insert_prepara() 
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
    BEFORE INSERT ON sucursales
    FOR EACH ROW
    EXECUTE FUNCTION sucursales_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION sucursales_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON sucursales
    FOR EACH ROW
    EXECUTE FUNCTION sucursales_update_prepara();
