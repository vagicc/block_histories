DROP TABLE IF EXISTS `mass_block_histories`;
CREATE TABLE `mass_block_histories` (
  `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'tempid',
  `tx_id` varchar(128) DEFAULT 'mass' COMMENT '交易ID:用来去钱包节点查到详细信息',
  `block_height` bigint NOT NULL COMMENT '爆块时区块高度',
  `block_time` BIGINT DEFAULT NULL COMMENT '爆块时间(超过12个小时，才会到账)',
  `amount` decimal(55, 12) DEFAULT '0' COMMENT '爆块1个块等于64个mass币,分配到了多少个币',
  `from_addresses` varchar(58) DEFAULT 'COINBASE' COMMENT '钱包爆块类型：COINBASE是爆块',
  `wallet_address` varchar(150) default null COMMENT '钱包地址',

  `pool_id` bigint NOT NULL DEFAULT 0 COMMENT '矿池ID',
  `pool_name` varchar(188) DEFAULT NULL COMMENT '矿池名:mass-A,mass-B',
  `pool_type` varchar(128) DEFAULT NULL COMMENT '矿池类型：bzz、pha、chia、mass等',
  `pool_address` varchar(255) DEFAULT NULL COMMENT '矿池地址，产币地址（出块地址）,用来对应矿机',
  `status` tinyint(2) NOT NULL DEFAULT 0 COMMENT '是否分币（0未分币，1已分币），是否记录到“矿池产币日志记录表coin_pool_daily_coin”：0 默认没记录，1已记录',
  `modify` datetime NOT NULL DEFAULT current_timestamp() COMMENT '修改时间',
  `create_time` datetime NOT NULL DEFAULT current_timestamp() COMMENT '插表创建时间',
  PRIMARY KEY (`id`),
  KEY `pool_id` (`pool_id`),
  KEY `block_time` (`block_time`),
  KEY `pool_address` (`pool_address`),
  KEY `wallet_address` (`wallet_address`),
  KEY `status` (`status`),
  KEY `tx_id` (`tx_id`)
) ENGINE = InnoDB COMMENT = 'mass爆块日志表';