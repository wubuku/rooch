package org.test.roochblogdemo;

import org.test.roochblogdemo.specialization.ApplicationContext;
import org.test.roochblogdemo.specialization.spring.SpringApplicationContext;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.autoconfigure.domain.EntityScan;
import org.springframework.boot.autoconfigure.security.servlet.SecurityAutoConfiguration;
import org.springframework.boot.context.event.ApplicationReadyEvent;
import org.springframework.context.ConfigurableApplicationContext;
import org.springframework.context.event.ContextStartedEvent;
import org.springframework.context.event.EventListener;
import org.springframework.scheduling.annotation.EnableScheduling;

//@EnableSwagger2
@SpringBootApplication(exclude = {
        SecurityAutoConfiguration.class
})
@EntityScan(basePackages = {
        "org.test.roochblogdemo.rooch.contract",
        "org.test.roochblogdemo.rooch.contract.persistence"
})
@EnableScheduling
//@EnableAutoConfiguration
public class RoochBlogDemoApplication {

    //@Autowired
    //private MoveObjectIdGeneratorObjectService moveObjectIdGeneratorObjectService;

    public static void main(String[] args) {
        ConfigurableApplicationContext ctx = SpringApplication.run(RoochBlogDemoApplication.class, args);
        ApplicationContext.current = new SpringApplicationContext(ctx);
        ctx.publishEvent(new ContextStartedEvent(ctx));
    }

    @EventListener(ApplicationReadyEvent.class)
    public void initMoveObjectIdGeneratorObjects() {
        //moveObjectIdGeneratorObjectService.initMoveObjectIdGeneratorObjects();
    }
}
