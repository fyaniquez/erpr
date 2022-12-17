--
-- PostgreSQL database dump
--

-- Dumped from database version 13.8 (Debian 13.8-0+deb11u1)
-- Dumped by pg_dump version 13.8 (Debian 13.8-0+deb11u1)

-- Started on 2022-12-16 18:38:51 -04

CREATE TABLE public.catalogos (
    id bigint NOT NULL,
    nombre TEXT NOT NULL,
    propietario bigint,
    empresa_id bigint,
    fecha TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    activo BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    CONSTRAINT ctl_pkey PRIMARY KEY(id)
);


ALTER TABLE public.catalogos OWNER TO erp;

--
-- TOC entry 3239 (class 0 OID 0)
-- Dependencies: 213
-- Name: COLUMN catalogos.propietario; Type: COMMENT; Schema: public; Owner: erp
--

COMMENT ON COLUMN public.catalogos.propietario IS 'empresa que compra/vende los productos indicados en el catalogo, si es igual empresa_id se trata del catalogo de la propia empresa';

COMMENT ON COLUMN public.catalogos.empresa_id IS 'empresa de la que depende el catalogo';

COMMENT ON COLUMN public.catalogos.fecha IS 'fecha desde la que es válida el catalogo';

CREATE SEQUENCE public.catalogos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER TABLE public.catalogos_id_seq OWNER TO erp;

ALTER SEQUENCE public.catalogos_id_seq OWNED BY public.catalogos.id;

ALTER TABLE ONLY public.catalogos ALTER COLUMN id SET DEFAULT nextval('public.catalogos_id_seq'::regclass);

--datos
INSERT INTO public.catalogos (id, nombre, activo, created_at, updated_at, propietario, empresa_id, fecha) VALUES (1, '2021', true, '2021-03-21 22:09:57.367588', '2021-04-02 15:21:32.988769', 1, 1, '2021-04-01 00:00:00');

-- ajusta sequencia
SELECT pg_catalog.setval('public.catalogos_id_seq', 1, true);

-- constraints
ALTER TABLE public.catalogos
    ADD CONSTRAINT ctl_fk_ctl_emp 
    FOREIGN KEY (empresa_id) REFERENCES public.empresas(id);

ALTER TABLE public.catalogos
    ADD CONSTRAINT ctl_fk_ctl_propietario
    FOREIGN KEY (propietario) REFERENCES public.empresas(id);

ALTER TABLE public.catalogos
    ADD CONSTRAINT ctl_uk_empresaid_propietario_nombre 
    UNIQUE (empresa_id, propietario, nombre);


-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION catalogos_insert_prepara() 
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
    BEFORE INSERT ON catalogos
    FOR EACH ROW
    EXECUTE FUNCTION catalogos_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION catalogos_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON catalogos
    FOR EACH ROW
    EXECUTE FUNCTION catalogos_update_prepara();

