from langchain.llms import OpenAI
from langchain.prompts import PromptTemplate
from langchain.chains import LLMChain

template = PromptTemplate.from_template("Classify this Solana batch job:\n{query}\n\nOutput: High Priority or Low Priority?")

llm = OpenAI(temperature=0.0)
chain = LLMChain(prompt=template, llm=llm)

def classify_batch(query: str):
    result = chain.run(query=query)
    print(f"Batch Classification: {result}")

