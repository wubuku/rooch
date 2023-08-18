// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.domain.article;

import java.util.Map;
import java.util.List;
import org.dddml.support.criterion.Criterion;
import java.math.BigInteger;
import java.util.Date;
import org.test.roochblogdemo.domain.*;

public interface ArticleStateQueryRepository {
    ArticleState get(String id);

    Iterable<ArticleState> getAll(Integer firstResult, Integer maxResults);
    
    Iterable<ArticleState> get(Iterable<Map.Entry<String, Object>> filter, List<String> orders, Integer firstResult, Integer maxResults);

    Iterable<ArticleState> get(Criterion filter, List<String> orders, Integer firstResult, Integer maxResults);

    ArticleState getFirst(Iterable<Map.Entry<String, Object>> filter, List<String> orders);

    ArticleState getFirst(Map.Entry<String, Object> keyValue, List<String> orders);

    Iterable<ArticleState> getByProperty(String propertyName, Object propertyValue, List<String> orders, Integer firstResult, Integer maxResults);

    long getCount(Iterable<Map.Entry<String, Object>> filter);

    long getCount(Criterion filter);

    CommentState getComment(String articleId, BigInteger commentSeqId);

    Iterable<CommentState> getComments(String articleId, Criterion filter, List<String> orders);

}

