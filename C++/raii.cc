void unsafeFunction() {
  Resource* resource = new Resource();

  /* Do something with resource */
  thisFunctionCanThrowException();

  /* Do something else with resource */
  delete resource;
}

void unmaintanableFunction() {
  Resource* resource;
  try {
    resource = new Resource();

    /* Do something with resource */
    thisFunctionCanThrowException();

    /* Do something else with resource */
    delete resource;
  catch(std::exception& e) {
    delete resource;
    throw e;
  }
}

void safeFunction() {
  std::unique_ptr<Resource> resource(new Resource());

  /* Do something with resource */
  thisFunctionCanThrowException();
  /* Do something else with resource */
}
