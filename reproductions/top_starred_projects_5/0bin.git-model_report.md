# Model report for file:///tmp/top-repos-quality-repos-djntt0u3/0bin.git HEAD c65d5c4d090d385eb5c40df8e0001af9267f3b4b

### Dump

```json
{'created_at': '2021-08-30 00:16:51',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.0 kB',
 'tags': [],
 'uuid': '725b2244-0b2b-481d-a38b-027ca5b189f4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-djntt0u3/0bin.git c65d5c4d090d385eb5c40df8e0001af9267f3b4b

# javascript
20 rules, avg.len. 6.2
## train
PPCR: 0.910862
### report
macro
{'f1-score': 0.6956068180628633,
 'precision': 0.6961474326773072,
 'recall': 0.6961283390488611,
 'support': 18281}
micro
{'f1-score': 0.9495104206553252,
 'precision': 0.9495104206553252,
 'recall': 0.9495104206553252,
 'support': 18281}
weighted
{'f1-score': 0.9428488479367283,
 'precision': 0.9370592939677324,
 'recall': 0.9495104206553252,
 'support': 18281}
### report_full
macro
{'f1-score': 0.6571711843393007,
 'precision': 0.6961474326773072,
 'recall': 0.624494469126494,
 'support': 20070}
micro
{'f1-score': 0.9052175953690907,
 'precision': 0.9495104206553252,
 'recall': 0.8648729446935725,
 'support': 20070}
weighted
{'f1-score': 0.8898529786665161,
 'precision': 0.9182511144787392,
 'recall': 0.8648729446935725,
 'support': 20070}
## test
PPCR: 0.882636
### report
macro
{'f1-score': 0.6697030185882631,
 'precision': 0.6972989080386244,
 'recall': 0.6503462189213118,
 'support': 1835}
micro
{'f1-score': 0.9313351498637602,
 'precision': 0.9313351498637602,
 'recall': 0.9313351498637602,
 'support': 1835}
weighted
{'f1-score': 0.9279915730101704,
 'precision': 0.9280225453854053,
 'recall': 0.9313351498637602,
 'support': 1835}
### report_full
macro
{'f1-score': 0.6144105273642471,
 'precision': 0.6972989080386244,
 'recall': 0.5573113077172774,
 'support': 2079}
micro
{'f1-score': 0.8732754215636178,
 'precision': 0.9313351498637602,
 'recall': 0.822029822029822,
 'support': 2079}
weighted
{'f1-score': 0.8597127746726851,
 'precision': 0.9123841221717959,
 'recall': 0.822029822029822,
 'support': 2079}
```

## javascript
### Summary
12 rules, avg.len. 5.6

| | |
|-|-|
|Min support|125|
|Max support|3404|
|Min confidence|0.9338942170143127|
|Max confidence|0.9985074400901794|

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
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 125.` |
| 2 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 239.` |
| 3 | `  -1.internal_type = Identifier<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 3404.` |
| 4 | `  -1.internal_type not in {Identifier}<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 1695.` |
| 5 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1078.` |
| 6 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ;<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 825.` |
| 7 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.934. Support: 416.` |
| 8 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.958. Support: 389.` |
| 9 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.959. Support: 283.` |
| 10 | `  -1.internal_type not in {CommentLine, Identifier}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 335.` |
| 11 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 286.` |
| 12 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 148.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.583333333333333, "max_conf": 0.9985074400901794, "max_support": 3404, "min_conf": 0.9338942170143127, "min_support": 125, "num_rules": 12}}
```
</details>
