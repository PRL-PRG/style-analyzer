# Model report for file:///tmp/top-repos-quality-repos-5avqt3ay/xiaochaoyue.git HEAD 5a5bc6959257a67a0aac97ccf895b9755310401e

### Dump

```json
{'created_at': '2021-08-21 03:59:42',
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
 'size': '18.2 kB',
 'tags': [],
 'uuid': 'e010f941-ffd5-4f8f-ba0f-fbdee1f0fba0',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-5avqt3ay/xiaochaoyue.git 5a5bc6959257a67a0aac97ccf895b9755310401e

# javascript
21 rules, avg.len. 9.4
## train
PPCR: 0.931755
### report
macro
{'f1-score': 0.7088672889154741,
 'precision': 0.7290166872272155,
 'recall': 0.693372159597748,
 'support': 36440}
micro
{'f1-score': 0.9515916575192097,
 'precision': 0.9515916575192097,
 'recall': 0.9515916575192097,
 'support': 36440}
weighted
{'f1-score': 0.9499253025960415,
 'precision': 0.9508470288182747,
 'recall': 0.9515916575192097,
 'support': 36440}
### report_full
macro
{'f1-score': 0.6354620874629986,
 'precision': 0.7290166872272155,
 'recall': 0.584577251218326,
 'support': 39109}
micro
{'f1-score': 0.9179737653708189,
 'precision': 0.9515916575192097,
 'recall': 0.8866501316832442,
 'support': 39109}
weighted
{'f1-score': 0.9071069474298245,
 'precision': 0.9412663858673098,
 'recall': 0.8866501316832442,
 'support': 39109}
## test
PPCR: 0.931706
### report
macro
{'f1-score': 0.7081038388775226,
 'precision': 0.7279096186223655,
 'recall': 0.692987332718273,
 'support': 36453}
micro
{'f1-score': 0.9509779716347077,
 'precision': 0.9509779716347077,
 'recall': 0.9509779716347077,
 'support': 36453}
weighted
{'f1-score': 0.9493016549522584,
 'precision': 0.9502506068470151,
 'recall': 0.9509779716347077,
 'support': 36453}
### report_full
macro
{'f1-score': 0.6349287443068188,
 'precision': 0.7279096186223655,
 'recall': 0.5842276031625129,
 'support': 39125}
micro
{'f1-score': 0.9173569028024028,
 'precision': 0.9509779716347077,
 'recall': 0.8860319488817892,
 'support': 39125}
weighted
{'f1-score': 0.9065002459167579,
 'precision': 0.9405976813338274,
 'recall': 0.8860319488817892,
 'support': 39125}
```

## javascript
### Summary
15 rules, avg.len. 8.0

| | |
|-|-|
|Min support|185|
|Max support|10930|
|Min confidence|0.9224137663841248|
|Max confidence|0.9995187520980835|

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
| 1 | `  -1.reserved not in {[}<br>	∧ +1.reserved not in {)}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 4131.` |
| 2 | `  +2.roles in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 1786.` |
| 3 | `  +1.reserved = ;<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.948. Support: 1567.` |
| 4 | `  -1.internal_type = CommentLine<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 948.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎⇥⁺<br>Confidence: 0.936. Support: 964.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {;, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1039.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 870.` |
| 8 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 562.` |
| 9 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {{}<br>	∧ +1.reserved not in {,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = "<br>Confidence: 0.999. Support: 477.` |
| 10 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {MAP}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 275.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = !<br>	∧ -1.roles not in {MAP}<br>	∧ +1.reserved not in {}}<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 188.` |
| 12 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 430.` |
| 13 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 185.` |
| 14 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {(, ), ,, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.923. Support: 2069.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {!, (, ;, {}<br>	∧ -1.roles not in {MAP}<br>	∧ -2.label not in {<-tab>}<br>	∧ +1.reserved not in {(, ,, ;, }}<br>	∧ +1.roles not in {COMMENT, KEY}<br>	∧ +2.roles not in {ARGUMENT, COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 10930.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.0, "max_conf": 0.9995187520980835, "max_support": 10930, "min_conf": 0.9224137663841248, "min_support": 185, "num_rules": 15}}
```
</details>
