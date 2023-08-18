// <autogenerated>
//   This file was generated by dddappp code generator.
//   Any changes made to this file manually will be lost next time the file is regenerated.
// </autogenerated>

package org.test.roochblogdemo.domain.blog.hibernate;

import java.util.*;
import java.util.Date;
import java.math.BigInteger;
import org.test.roochblogdemo.domain.*;
import org.hibernate.Session;
import org.hibernate.Criteria;
//import org.hibernate.criterion.Order;
import org.hibernate.criterion.Criterion;
import org.hibernate.criterion.Projections;
import org.hibernate.SessionFactory;
import org.test.roochblogdemo.domain.blog.*;
import org.test.roochblogdemo.specialization.*;
import org.test.roochblogdemo.specialization.hibernate.*;
import org.springframework.transaction.annotation.Transactional;

public class HibernateBlogStateRepository implements BlogStateRepository {
    private SessionFactory sessionFactory;

    public SessionFactory getSessionFactory() { return this.sessionFactory; }

    public void setSessionFactory(SessionFactory sessionFactory) { this.sessionFactory = sessionFactory; }

    protected Session getCurrentSession() {
        return this.sessionFactory.getCurrentSession();
    }
    
    private static final Set<String> readOnlyPropertyPascalCaseNames = new HashSet<String>(Arrays.asList("AccountAddress", "Name", "Articles", "OffChainVersion", "CreatedBy", "CreatedAt", "UpdatedBy", "UpdatedAt", "Active", "Deleted", "Version"));
    
    private ReadOnlyProxyGenerator readOnlyProxyGenerator;
    
    public ReadOnlyProxyGenerator getReadOnlyProxyGenerator() {
        return readOnlyProxyGenerator;
    }

    public void setReadOnlyProxyGenerator(ReadOnlyProxyGenerator readOnlyProxyGenerator) {
        this.readOnlyProxyGenerator = readOnlyProxyGenerator;
    }

    @Transactional(readOnly = true)
    public BlogState get(String id, boolean nullAllowed) {
        BlogState.SqlBlogState state = (BlogState.SqlBlogState)getCurrentSession().get(AbstractBlogState.SimpleBlogState.class, id);
        if (!nullAllowed && state == null) {
            state = new AbstractBlogState.SimpleBlogState();
            state.setAccountAddress(id);
        }
        if (getReadOnlyProxyGenerator() != null && state != null) {
            return (BlogState) getReadOnlyProxyGenerator().createProxy(state, new Class[]{BlogState.SqlBlogState.class}, "getStateReadOnly", readOnlyPropertyPascalCaseNames);
        }
        return state;
    }

    public void save(BlogState state) {
        BlogState s = state;
        if (getReadOnlyProxyGenerator() != null) {
            s = (BlogState) getReadOnlyProxyGenerator().getTarget(state);
        }
        if(s.getOffChainVersion() == null) {
            getCurrentSession().save(s);
        } else {
            getCurrentSession().update(s);
        }

        if (s instanceof Saveable)
        {
            Saveable saveable = (Saveable) s;
            saveable.save();
        }
        getCurrentSession().flush();
    }

    public void merge(BlogState detached) {
        BlogState persistent = getCurrentSession().get(AbstractBlogState.SimpleBlogState.class, detached.getAccountAddress());
        if (persistent != null) {
            merge(persistent, detached);
            getCurrentSession().merge(detached);
        } else {
            getCurrentSession().save(detached);
        }
        getCurrentSession().flush();
    }

    private void merge(BlogState persistent, BlogState detached) {
        ((BlogState.MutableBlogState) detached).setOffChainVersion(persistent.getOffChainVersion());
    }

}

