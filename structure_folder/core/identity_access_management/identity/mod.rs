/*
   Доменный слой.
   Ядро бизнес-логики. Содержит агрегаты (Order Aggregate, Product Aggregate), сущности, объекты-значения и доменные события.
   Не зависит от внешних слоев. Order Read Model и Product Read Model представляют собой оптимизированные для чтения
   представления данных, используемые Query Handlers.
*/
pub mod aggregate;
pub mod event;
pub mod ports;
pub mod service;
