package de.fhg.aisec.ids.clearinghouse;

import javax.net.ssl.*;
import java.net.Socket;
import java.security.*;
import java.security.cert.*;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ChTrustManager extends X509ExtendedTrustManager {

    final static Logger LOG = LoggerFactory.getLogger(ChTrustManager.class);

    ChTrustManager() throws Exception {
        LOG.info("received no keystore");
    }

    ChTrustManager(KeyStore keystore) throws Exception {
        LOG.info("received keystore: {}", keystore.aliases());
    }

    @Override
    public void checkClientTrusted(X509Certificate[] chain, String authType, Socket socket) throws CertificateException {
    }

    @Override
    public void checkServerTrusted(X509Certificate[] chain, String authType, Socket socket) throws CertificateException {

    }

    @Override
    public void checkClientTrusted(X509Certificate[] chain, String authType, SSLEngine engine) throws CertificateException {

    }

    @Override
    public void checkServerTrusted(X509Certificate[] chain, String authType, SSLEngine engine) throws CertificateException {

    }

    @Override
    public void checkClientTrusted(X509Certificate[] chain, String authType) throws CertificateException {

    }

    @Override
    public void checkServerTrusted(X509Certificate[] chain, String authType) throws CertificateException {

    }

    @Override
    public X509Certificate[] getAcceptedIssuers() {
        return new X509Certificate[0];
    }
}
