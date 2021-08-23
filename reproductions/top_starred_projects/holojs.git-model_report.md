# Model report for file:///tmp/top-repos-quality-repos-7sr8mb6_/holojs.git HEAD bdb7afb0d45a858f45a5a17b15b98291a96e7f13

### Dump

```json
{'created_at': '2021-08-23 06:32:30',
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
 'size': '20.3 kB',
 'tags': [],
 'uuid': '8a4b811b-dd1f-4599-bf44-2a153e2909de',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-7sr8mb6_/holojs.git bdb7afb0d45a858f45a5a17b15b98291a96e7f13

# javascript
23 rules, avg.len. 6.3
## train
PPCR: 0.888799
### report
macro
{'f1-score': 0.4965859780123561,
 'precision': 0.506939649343979,
 'recall': 0.49146909039555153,
 'support': 38421}
micro
{'f1-score': 0.9637177585174774,
 'precision': 0.9637177585174774,
 'recall': 0.9637177585174774,
 'support': 38421}
weighted
{'f1-score': 0.9584591744262413,
 'precision': 0.9546708757479078,
 'recall': 0.9637177585174774,
 'support': 38421}
### report_full
macro
{'f1-score': 0.4449884438631422,
 'precision': 0.506939649343979,
 'recall': 0.413747666295381,
 'support': 43228}
micro
{'f1-score': 0.9069798772795747,
 'precision': 0.9637177585174774,
 'recall': 0.8565513093365411,
 'support': 43228}
weighted
{'f1-score': 0.8783176038670607,
 'precision': 0.9118344550973742,
 'recall': 0.8565513093365411,
 'support': 43228}
## test
PPCR: 0.879571
### report
macro
{'f1-score': 0.465867722071092,
 'precision': 0.4666060169401032,
 'recall': 0.47818598458175837,
 'support': 11642}
micro
{'f1-score': 0.9222642157704861,
 'precision': 0.9222642157704861,
 'recall': 0.9222642157704861,
 'support': 11642}
weighted
{'f1-score': 0.9170283520691531,
 'precision': 0.9187549359297602,
 'recall': 0.9222642157704861,
 'support': 11642}
### report_full
macro
{'f1-score': 0.40550370046118156,
 'precision': 0.4666060169401032,
 'recall': 0.3823090118308343,
 'support': 13236}
micro
{'f1-score': 0.8631722807299622,
 'precision': 0.9222642157704861,
 'recall': 0.8111967361740707,
 'support': 13236}
weighted
{'f1-score': 0.8335939300819997,
 'precision': 0.8741597045330658,
 'recall': 0.8111967361740707,
 'support': 13236}
```

## javascript
### Summary
14 rules, avg.len. 5.7

| | |
|-|-|
|Min support|150|
|Max support|9737|
|Min confidence|0.9224599003791809|
|Max confidence|0.9996938109397888|

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
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 185,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 8390.` |
| 2 | `  +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2546.` |
| 3 | `  -1.reserved = ;<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 167.` |
| 4 | `  -1.roles in {STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 160.` |
| 5 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1091.` |
| 6 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1633.` |
| 7 | `  -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = (<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 731.` |
| 8 | `  -1.reserved = {<br>	∧ -4.label in {<space>}<br>	∧ +1.reserved not in {(, ,}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⏎⇥⁺<br>Confidence: 0.922. Support: 561.` |
| 9 | `  -1.reserved not in {;, {}<br>	∧ +1.reserved not in {(, ;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 464.` |
| 10 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.reserved not in {(, ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 253.` |
| 11 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {KEY} and not in {COMMENT}<br>	∧ +1.reserved not in {(}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 302.` |
| 12 | `  •••start_line ≥ 208<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 150.` |
| 13 | `  •••start_line ≥ 225<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {COMMENT, KEY}<br>	∧ +1.reserved not in {(, ), ,}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 1173.` |
| 14 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {COMMENT, KEY}<br>	∧ +1.reserved not in {(, ), ,, ;}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 9737.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.714285714285714, "max_conf": 0.9996938109397888, "max_support": 9737, "min_conf": 0.9224599003791809, "min_support": 150, "num_rules": 14}}
```
</details>
