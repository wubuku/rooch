// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.domain.article;

import java.util.*;
import java.math.BigInteger;
import java.util.Date;
import org.test.roochblogdemo.domain.*;
import org.test.roochblogdemo.domain.AbstractCommand;

public abstract class AbstractArticleCommand extends AbstractCommand implements ArticleCommand {

    private String id;

    public String getId()
    {
        return this.id;
    }

    public void setId(String id)
    {
        this.id = id;
    }

    private Long offChainVersion;

    public Long getOffChainVersion()
    {
        return this.offChainVersion;
    }

    public void setOffChainVersion(Long offChainVersion)
    {
        this.offChainVersion = offChainVersion;
    }


}

