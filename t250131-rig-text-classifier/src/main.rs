// Import necessary dependencies
use rig::providers::openai;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define enum for sentiment classification
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

// Define an enum for sentiment confidence level
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct SentimentClassification {
    sentiment: Sentiment,
    confidence: f32,
}

// Define an enum to represent news article topics
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
enum Topic {
    Politics,
    Technology,
    Sports,
    Entertainment,
    Other(String),
}

// Define a struct to hold the news article classification result
#[derive(Debug, Deserialize, JsonSchema, Serialize)]
struct NewsArticleClassification {
    topic: Topic,
    sentiment: SentimentClassification,
    summary: String,
}

fn pretty_print_result(article: &str, result: &NewsArticleClassification) {
    println!("Article: \"{}...\"", &article[..100]); // Print first 100 characters
    println!("Classification Result:");
    println!("  Topic: {:?}", result.topic);
    println!("  Sentiment: {:?}", result.sentiment.sentiment);
    println!("  Confidence: {:.2}%", result.sentiment.confidence * 100.0);
    println!("  Summary: {}", result.summary);
    println!();
}

#[tokio::main]
async fn main() {
    // Initialize the OpenAI client
    let openai_client = openai::Client::from_env();

    // Create a news article classifier using Rig's Extractor
    let news_classifier = openai_client
        .extractor::<NewsArticleClassification>("gpt-4")
        .preamble("
            You are a news article classification AI. For the given news article:
            1. Classify the main topic (Politics, Technology, Sports, Entertainment, or Other).
            2. Analyze the overall sentiment (Positive, Negative, or Neutral) with a confidence score.
            3. Provide a brief summary of the article.
        ")
        .build();

    // Sample news article to classify
    let article = "
        After conducting the first-ever commercial spacewalk and traveling farther from Earth than anyone \
        in more than half a century, the astronauts of the Polaris Dawn mission returned to Earth safely \
        early Sunday.

        The SpaceX Crew Dragon capsule splashed down in the Gulf of Mexico, off the coast of Dry Tortugas, \
        Fla., shortly after 3:30 a.m., carrying Jared Isaacman, a billionaire entrepreneur, and his crew \
        of three private astronauts, according to a SpaceX livestream.

        The ambitious space mission, a collaboration between Mr. Isaacman and Elon Musk's SpaceX, spent \
        five days in orbit, achieved several milestones in private spaceflight and was further evidence \
        that space travel and spacewalks are no longer the exclusive domain of professional astronauts \
        working at government agencies like NASA.

        The Crew Dragon capsule launched on Tuesday, after delays because of a helium leak and bad weather. \
        On board were Mr. Isaacman, the mission commander and the founder of the payment services company \
        Shift4; Sarah Gillis and Anna Menon, SpaceX employees; and Scott Poteet, a retired U.S. Air Force \
        lieutenant colonel.

        Late on Tuesday, its orbit reached a high point of about 870 miles above the Earth's surface. That \
        beat the record distance for astronauts on a mission not headed to the moon, which the Gemini XI \
        mission set in 1966 at 853 miles high, and made Ms. Gillis and Ms. Menon the first women ever to \
        fly so far from Earth.

        On Thursday, Mr. Isaacman and Ms. Gillis became the first private astronauts to successfully complete \
        a spacewalk. The operation involved the crew letting all the air out of the spacecraft, because it \
        had no airlock, while the other two crew members wore spacesuits inside the airless capsule. Mr. \
        Isaacman moved outside and conducted mobility tests of his spacesuit for a few minutes before \
        re-entering the capsule. Ms Gillis then moved outside and performed the same tests.

        This was the first of three Polaris missions aimed at accelerating technological advances needed to \
        fulfill Mr. Musk's dream of sending people to Mars someday. A key goal of the mission was to further \
        the development of more advanced spacesuits that would be needed for SpaceX to try any future \
        off-world colonization.

        During a news conference before the launch, Mr. Isaacman mused that one day, someone might step onto \
        Mars wearing a version of the spacesuit that SpaceX had developed for this flight. Closer to Earth, \
        commercial spacewalks also present other possibilities, like technicians repairing private satellites \
        in orbit.

        During the spaceflight, the four astronauts conducted about 40 experiments, mostly about how \
        weightlessness and radiation affect the human body. They also tested laser communications between \
        the Crew Dragon and SpaceX's constellation of Starlink internet satellites.\
    ";

    // Perform news article classification
    match news_classifier.extract(article).await {
        Ok(result) => pretty_print_result(article, &result),
        Err(e) => eprintln!("Error classifying article: {}", e),
    }
}
