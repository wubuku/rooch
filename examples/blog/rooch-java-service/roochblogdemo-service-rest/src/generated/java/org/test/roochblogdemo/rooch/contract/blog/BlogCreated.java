// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.rooch.contract.blog;

import com.fasterxml.jackson.databind.PropertyNamingStrategies;
import com.fasterxml.jackson.databind.annotation.JsonNaming;

import org.test.roochblogdemo.rooch.contract.*;

import java.math.*;
import java.util.*;

@JsonNaming(PropertyNamingStrategies.SnakeCaseStrategy.class)
public class BlogCreated {
    private com.github.wubuku.rooch.bean.AnnotatedMoveOptionView<String> id;

    private String accountAddress;

    private String name;

    private String[] articles;

    public com.github.wubuku.rooch.bean.AnnotatedMoveOptionView<String> getId() {
        return id;
    }

    public void setId(com.github.wubuku.rooch.bean.AnnotatedMoveOptionView<String> id) {
        this.id = id;
    }

    public String getAccountAddress() {
        return accountAddress;
    }

    public void setAccountAddress(String accountAddress) {
        this.accountAddress = accountAddress;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String[] getArticles() {
        return articles;
    }

    public void setArticles(String[] articles) {
        this.articles = articles;
    }

    @Override
    public String toString() {
        return "BlogCreated{" +
                "id='" + id + '\'' +
                ", accountAddress=" + '\'' + accountAddress + '\'' +
                ", name=" + '\'' + name + '\'' +
                ", articles=" + Arrays.toString(articles) +
                '}';
    }

}
