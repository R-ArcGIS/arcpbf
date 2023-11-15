# list of response tables

    Code
      resps_data_pbf(resps)
    Output
        OBJECTID       Adoption_Service_Provider  DBA       city                state
      1        1 A Family in Bloom Adoption, LLC <NA>    Boulder             Colorado
      2        2     A Love Beyond Borders, Inc. <NA>     Denver             Colorado
      3        3                  Adopolis, Inc. <NA> Washington District of Columbia
      4        1 A Family in Bloom Adoption, LLC <NA>    Boulder             Colorado
      5        2     A Love Beyond Borders, Inc. <NA>     Denver             Colorado
      6        3                  Adopolis, Inc. <NA> Washington District of Columbia
      7        1 A Family in Bloom Adoption, LLC <NA>    Boulder             Colorado
      8        2     A Love Beyond Borders, Inc. <NA>     Denver             Colorado
      9        3                  Adopolis, Inc. <NA> Washington District of Columbia
         Accreditation_or_Approval_Statu Accredited_Approvedto_Provide
      1 Accredited/Approved – In Process         Incoming and Outgoing
      2                         Approved                      Incoming
      3                       Accredited                      Incoming
      4 Accredited/Approved – In Process         Incoming and Outgoing
      5                         Approved                      Incoming
      6                       Accredited                      Incoming
      7 Accredited/Approved – In Process         Incoming and Outgoing
      8                         Approved                      Incoming
      9                       Accredited                      Incoming
                                                full_address
      1           Adoption Service Provider,Boulder,Colorado
      2      A Family in Bloom Adoption, LLC,Denver,Colorado
      3 A Love Beyond Borders, Inc.,Washington,Washington DC
      4           Adoption Service Provider,Boulder,Colorado
      5      A Family in Bloom Adoption, LLC,Denver,Colorado
      6 A Love Beyond Borders, Inc.,Washington,Washington DC
      7           Adoption Service Provider,Boulder,Colorado
      8      A Family in Bloom Adoption, LLC,Denver,Colorado
      9 A Love Beyond Borders, Inc.,Washington,Washington DC

# test list of feature classes

    Code
      resps_data_pbf(resps)
    Output
      Simple feature collection with 6 features and 1 field
      Geometry type: POLYGON
      Dimension:     XY
      Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17260020 ymax: 2261306
      Projected CRS: WGS 84 / Pseudo-Mercator
               County                       geometry
      1 Hawaii County POLYGON ((-17264972 2244291...
      2 Hawaii County POLYGON ((-17264972 2244291...
      3 Hawaii County POLYGON ((-17264972 2244291...
      4 Hawaii County POLYGON ((-17264972 2244291...
      5 Hawaii County POLYGON ((-17264972 2244291...
      6 Hawaii County POLYGON ((-17264972 2244291...

# test list of OIDs

    Code
      resps_data_pbf(resps)
    Output
        OBJECTID
      1        1
      2        2
      3        3
      4        1
      5        2
      6        3
      7        1
      8        2
      9        3

# test list of counts

    Code
      resps_data_pbf(resps)
    Output
      [1] 3143 3143 3143

