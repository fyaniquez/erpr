--

CREATE TABLE public.puestos (
    id bigint NOT NULL,
    nombre text NOT NULL,
    sigla text NOT  NULL,
    descripcion text NOT NULL,
    sucursal_id bigint NOT NULL,
    activo boolean DEFAULT false NOT NULL,
    created_at timestamp without time zone NOT NULL,
    updated_at timestamp without time zone NOT NULL
);


ALTER TABLE public.puestos OWNER TO erp;

COMMENT ON TABLE public.puestos IS 'puestos físicos de trabajo habilitados en la sucursal';

COMMENT ON COLUMN public.puestos.nombre IS 'nombre con el que aparece en los registros';

COMMENT ON COLUMN public.puestos.descripcion IS 'descripción detallada del puesto puede tener el cargo ';

COMMENT ON COLUMN public.puestos.sigla IS 'nombre corto del punto para vouchers';

COMMENT ON COLUMN public.puestos.sucursal_id IS 'sucursal en la que el puesto físico se encuentra';

COMMENT ON COLUMN public.puestos.activo IS 'bandera para permitir transacciones desde este puesto';

CREATE SEQUENCE public.puestos_id_seq
    AS bigint
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.puestos_id_seq OWNER TO erp;

ALTER SEQUENCE public.puestos_id_seq OWNED BY public.puestos.id;

ALTER TABLE ONLY public.puestos ALTER COLUMN id SET DEFAULT nextval('public.puestos_id_seq'::regclass);

INSERT INTO public.puestos (id, nombre, descripcion, sucursal_id, activo, created_at, updated_at, sigla) VALUES (1, 'Caja 1', 'Punto de venta ubicado en la tienda de calle 19', 1, true, '2022-07-22 08:00:00', '2022-07-22 08:00:00', 'CJ01');
INSERT INTO public.puestos (id, nombre, descripcion, sucursal_id, activo, created_at, updated_at, sigla) VALUES (2, 'Inventariador 1', 'Inventariador de la sucursal 1', 1, true, '2022-07-22 08:00:00', '2022-07-22 08:00:00', 'IN01');

SELECT pg_catalog.setval('public.puestos_id_seq', 2, true);

ALTER TABLE ONLY public.puestos
    ADD CONSTRAINT puestos_pkey PRIMARY KEY (id);

CREATE INDEX index_puestos_on_sucursal_id 
    ON public.puestos USING btree (sucursal_id);
ALTER TABLE ONLY public.puestos
    ADD CONSTRAINT fk_puestos_sucursales FOREIGN KEY (sucursal_id) 
    REFERENCES public.sucursales(id) ON UPDATE RESTRICT ON DELETE RESTRICT;

ALTER TABLE public.puestos
    ADD CONSTRAINT uk_puesto_nombre 
    UNIQUE (sucursal_id, nombre);

ALTER TABLE public.puestos
    ADD CONSTRAINT uk_puesto_sigla 
    UNIQUE (sucursal_id, sigla);

-- triggers
-- trigger para preparar el update
CREATE OR REPLACE FUNCTION puestos_insert_prepara() 
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
    BEFORE INSERT ON puestos
    FOR EACH ROW
    EXECUTE FUNCTION puestos_insert_prepara();

-- trigger para preparar el update
CREATE OR REPLACE FUNCTION puestos_update_prepara() 
   RETURNS TRIGGER 
   LANGUAGE PLPGSQL
AS $$
BEGIN
	NEW.updated_at = now();
    RETURN NEW;
END;
$$;

CREATE TRIGGER update_prepara
    BEFORE UPDATE ON puestos
    FOR EACH ROW
    EXECUTE FUNCTION puestos_update_prepara();
