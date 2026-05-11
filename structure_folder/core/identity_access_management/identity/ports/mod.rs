/*
* Исходящий порт(Outgoing Port). Определяется что нужно домену от внешнего мира.
* Реализуется в Outgoing Adapter в инфраструктуром слое.
* Эти порты используются Application слоем через сервисы.
*/
pub mod external_service_ports;
pub mod message_bus_ports;
pub mod read_repository_ports;
pub mod write_repository_ports;
