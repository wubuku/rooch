// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.domain.article;

import java.math.BigInteger;
import java.util.Date;
import org.test.roochblogdemo.domain.*;

public interface CommentEventDao {
    void save(CommentEvent e);

    Iterable<CommentEvent> findByArticleEventId(ArticleEventId articleEventId);

}

