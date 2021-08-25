# Model report for file:///tmp/top-repos-quality-repos-f4lo97qx/algorithm.git HEAD 4cd2f5ca51357494fafd058c105264bd8f02a7b3

### Dump

```json
{'created_at': '2021-08-22 06:06:31',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '15.2 kB',
 'tags': [],
 'uuid': '77acf70b-ac0f-4a6b-bcac-6faeea48df39',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-f4lo97qx/algorithm.git 4cd2f5ca51357494fafd058c105264bd8f02a7b3

# javascript
28 rules, avg.len. 4.9
## train
PPCR: 0.824392
### report
macro
{'f1-score': 0.29093841040540397,
 'precision': 0.28327841442285767,
 'recall': 0.29903399145508275,
 'support': 3934}
micro
{'f1-score': 0.8546009150991357,
 'precision': 0.8546009150991357,
 'recall': 0.8546009150991357,
 'support': 3934}
weighted
{'f1-score': 0.8305189943915693,
 'precision': 0.8077799136071325,
 'recall': 0.8546009150991357,
 'support': 3934}
### report_full
macro
{'f1-score': 0.27397187310272053,
 'precision': 0.28327841442285767,
 'recall': 0.2656292448792077,
 'support': 4772}
micro
{'f1-score': 0.7723409143119688,
 'precision': 0.8546009150991357,
 'recall': 0.7045264040234702,
 'support': 4772}
weighted
{'f1-score': 0.7222700712640678,
 'precision': 0.7419471214324268,
 'recall': 0.7045264040234702,
 'support': 4772}
## test
PPCR: 0.827782
### report
macro
{'f1-score': 0.30232239162219565,
 'precision': 0.29399040709714486,
 'recall': 0.3114625443042913,
 'support': 1019}
micro
{'f1-score': 0.887144259077527,
 'precision': 0.887144259077527,
 'recall': 0.887144259077527,
 'support': 1019}
weighted
{'f1-score': 0.8658963451536871,
 'precision': 0.846422568575714,
 'recall': 0.887144259077527,
 'support': 1019}
### report_full
macro
{'f1-score': 0.28623068610945374,
 'precision': 0.29399040709714486,
 'recall': 0.2788821810772353,
 'support': 1231}
micro
{'f1-score': 0.8035555555555557,
 'precision': 0.887144259077527,
 'recall': 0.7343623070674249,
 'support': 1231}
weighted
{'f1-score': 0.7544249587701852,
 'precision': 0.7756483307181419,
 'recall': 0.7343623070674249,
 'support': 1231}
```

## javascript
### Summary
15 rules, avg.len. 4.7

| | |
|-|-|
|Min support|182|
|Max support|921|
|Min confidence|0.9204545617103577|
|Max confidence|0.9982269406318665|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'max_depth': 10,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 748.` |
| 2 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 874.` |
| 3 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 282.` |
| 4 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 227.` |
| 5 | `  -3.length ≥ 3<br>	∧ ^1.roles in {VARIABLE} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 262.` |
| 6 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 921.` |
| 7 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 272.` |
| 8 | `  •••start_col ≥ 4<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 202.` |
| 9 | `  -1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 842.` |
| 10 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 266.` |
| 11 | `  -1.diff_col ≤ 2<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 12 | `  •••start_col ≤ 3<br>	∧ -1.diff_col ≤ 2<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 193.` |
| 13 | `  -1.roles not in {EXPRESSION}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 271.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 15 | `  •••start_col ≤ 3<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.920. Support: 220.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.666666666666667, "max_conf": 0.9982269406318665, "max_support": 921, "min_conf": 0.9204545617103577, "min_support": 182, "num_rules": 15}}
```
</details>
