use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::stream_consumer::StreamConsumer;
use rdkafka::consumer::{Consumer, ConsumerContext, Rebalance};
use rdkafka::error::KafkaResult;
use rdkafka::{ClientConfig, TopicPartitionList, ClientContext};
use std::{io::Error, io::ErrorKind};

pub struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        log::info!("Pre rebalance {:?}", rebalance);
    }

    fn post_rebalance(&self, rebalance: &Rebalance) {
        log::info!("Post rebalance {:?}", rebalance);
    }

    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        log::info!("Committing offsets: {:?}", result);
    }
}

// A type alias with your custom consumer can be created for convenience.
type LoggingConsumer = StreamConsumer<CustomContext>;

pub fn subscribe_topics(consumer: LoggingConsumer, topics: &[&str]) {
	loop {
		match consumer.subscribe(&topics.to_vec()) {
			Ok(value) => {
				log::info!("Subscribed to topics: {:?}", value);
				break
			},
			Err(e) => {
				log::warn!("Can't subscribe to specified topics '{:?}': {}", topics, e);
			},
		}
	}
}

pub fn create_consumer(brokers: &str, group_id: &str) -> Result<StreamConsumer<CustomContext>, Error> {
	let context: CustomContext = CustomContext;

	let consumer_value : KafkaResult<LoggingConsumer> = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
		.set("message.max.bytes", "1000000000")
		.set("message.copy.max.bytes", "1000000000")
		.set("receive.message.max.bytes", "2147483647")
		.set("socket.send.buffer.bytes", "100000000")
		.set("socket.receive.buffer.bytes", "100000000")
		.set("queued.max.messages.kbytes", "2097151")
		.set("fetch.message.max.bytes", "1000000000")
		.set("max.partition.fetch.bytes", "1000000000")
		.set("fetch.max.bytes", "2147483135")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(context);

	match consumer_value {
		Ok(value) => Ok(value),
		Err(error) => Err(Error::new(ErrorKind::InvalidData.into(), error))
	}
    // consumer
}
